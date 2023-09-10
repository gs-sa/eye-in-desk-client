use std::time::Duration;

use crate::config::EIDConfig;

use super::{Aruco, EyeInDesk};
use anyhow::{Error, Ok};
use nalgebra::{Const, Matrix3, Point2, SMatrix, Vector2};
use tokio::time::sleep;

pub fn find_homography(matches: Vec<(Vector2<f64>, Vector2<f64>)>) -> anyhow::Result<Matrix3<f64>> {
    // TODO detect degenerate cases
    let (m1, m2): (Vec<_>, Vec<_>) = matches.iter().map(|m| (m.0, m.1)).unzip();

    let count = m1.len();
    let mut c2 = Point2::<f64>::origin();
    let mut c1 = Point2::<f64>::origin();

    for i in 0..count {
        c2.x += m2[i].x;
        c2.y += m2[i].y;
        c1.x += m1[i].x;
        c1.y += m1[i].y;
    }

    c2.x /= count as f64;
    c2.y /= count as f64;
    c1.x /= count as f64;
    c1.y /= count as f64;

    let mut s2 = Point2::<f64>::origin();
    let mut s1 = Point2::<f64>::origin();

    for i in 0..count {
        s2.x += (c2.x - m2[i].x).abs();
        s2.y += (c2.y - m2[i].y).abs();
        s1.x += (c1.x - m1[i].x).abs();
        s1.y += (c1.y - m1[i].y).abs();
    }

    if s2.x.abs() < f64::EPSILON
        || s2.y.abs() < f64::EPSILON
        || s1.x.abs() < f64::EPSILON
        || s1.y.abs() < f64::EPSILON
    {
        return Err(Error::msg("Points are too close to each other"));
    }

    s2.x = count as f64 / s2.x;
    s2.y = count as f64 / s2.y;
    s1.x = count as f64 / s1.x;
    s1.y = count as f64 / s1.y;

    let inv_h_norm = Matrix3::new(1. / s2.x, 0., c2.x, 0., 1. / s2.y, c2.y, 0., 0., 1.);
    let h_norm2 = Matrix3::new(s1.x, 0., -c1.x * s1.x, 0., s1.y, -c1.y * s1.y, 0., 0., 1.);

    let mut ltl: SMatrix<f64, 9, 9> = SMatrix::zeros();
    for i in 0..count {
        let x2 = (m2[i].x - c2.x) * s2.x;
        let y2 = (m2[i].y - c2.y) * s2.y;
        let x1 = (m1[i].x - c1.x) * s1.x;
        let y1 = (m1[i].y - c1.y) * s1.y;
        let lx = [x1, y1, 1., 0., 0., 0., -x2 * x1, -x2 * y1, -x2];
        let ly = [0., 0., 0., x1, y1, 1., -y2 * x1, -y2 * y1, -y2];
        // println!("{} lx {:?} ly {:?}", i, lx, ly);
        for j in 0..9 {
            for k in 0..9 {
                ltl[(j, k)] += lx[j] * lx[k] + ly[j] * ly[k];
            }
        }
    }

    ltl.fill_lower_triangle_with_upper_triangle();
    let eigen = ltl.symmetric_eigen();

    let (eigen_vector_idx, _) = eigen.eigenvalues.argmin();
    let h0 = eigen.eigenvectors.column(eigen_vector_idx);
    let h0 = h0
        .clone_owned()
        .reshape_generic(Const::<3>, Const::<3>)
        .transpose();

    let res = (inv_h_norm * h0) * h_norm2;
    let res = res * (1.0 / res[(2, 2)]);

    Ok(res)
}

impl EyeInDesk {
    pub async fn calibration(&mut self) -> anyhow::Result<()> {
        // let mut eid = EyeInDesk::default_connect().await;
        self.clear_and_draw().await?;
        let (x_max, y_max) = self.get_drawable_size().await?;
        println!("x max:{x_max}\ny max:{y_max}");
        let padding = 50.;
        let step = 2;
        let size = 200.;

        let x_inc = (x_max - size - 2. * padding) / step as f64;
        let y_inc = (y_max - size - 2. * padding) / step as f64;

        let mut data = vec![];
        for x_index in 0..step {
            for y_index in 0..step {
                let x = x_inc * x_index as f64 + padding;
                let y = y_inc * y_index as f64 + padding;
                println!("x:{x}");
                println!("y:{y}");
                self.place_arucos(vec![Aruco {
                    x: x as f32,
                    y: y as f32,
                    size: size as f32,
                }])
                .await?;
                self.clear_and_draw().await?;
                sleep(Duration::from_secs_f64(1.)).await;
                loop {
                    let arucos = self.get_arucos().await?;
                    if let Some(a) = arucos.iter().find(|a| a.id == 0) {
                        data.push((
                            Vector2::new(a.x as f64, a.y as f64),
                            Vector2::new((x + size / 2.) as f64, (y + size / 2.) as f64),
                        ));
                        break;
                    }
                }
            }
        }
        self.clear_and_draw().await?;
        // println!("{data:?}");
        for (src, _dst) in data.iter() {
            print!("{}, {},", src.x, src.y);
        }
        println!();
        for (_src, dst) in data.iter() {
            print!("{}, {},", dst.x, dst.y);
        }
        let m = find_homography(data)?;
        // eid.
        // println!("{m}");
        // self.config.to_json().write_defalut().unwrap();
        let new_config = EIDConfig::from_matrix(m);
        new_config.to_json().write_defalut()?;
        self.config = new_config;
        Ok(())
    }
}
