use eye_in_desk_client::{EyeInDesk, Object};
use nalgebra::{Matrix3, Vector3};

struct ArucoMem {
    arucos: (f32, f32, f32),
    time_count: usize,
    spear_time: usize,
}

impl ArucoMem {
    fn update(&mut self, arucos: (f32, f32, f32)) {
        self.arucos = arucos;
        self.time_count = 0;
    }
    fn update_nothing(&mut self) {
        self.time_count += 1;
    }
    fn is_forget(&self) -> bool {
        self.time_count >= self.spear_time
    }
}

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    let mut eid1 = eid.clone();
    
    tokio::spawn(async move {
        loop {
            let state = eid1.get_real_robot_state().await.unwrap();
            eid1.update_virtual_robot(state.joints).await.unwrap();
        }
    });
    
    let mut m = Matrix3::<f32>::identity();
    m[(0, 0)] = -0.00054;
    m[(1, 1)] = -0.00054;
    m[(0, 2)] = 1.08;
    m[(1, 2)] = 0.287;

    let mut mem = ArucoMem {
        arucos: (0.,0.,0.),
        time_count: 0,
        spear_time: 20,
    };
    loop { 
        let arucos = eid.get_arucos().await.unwrap();
        if let Some(a) = arucos.iter().find(|a| a.id == 0) {
            let v = m * Vector3::new(a.x, a.y, 1.);
            mem.update((v.x, v.y, a.rot));
        } else {
            mem.update_nothing();
        }

        if !mem.is_forget() {
            eid.update_virtual_objects(vec![
                Object {
                    id: 5,
                    x: mem.arucos.0,
                    y: mem.arucos.1,
                    z: 0.025,
                    rot: mem.arucos.2,
                    scale: 1.,
                }
            ]).await.unwrap();
        } else {
            eid.update_virtual_objects(vec![]).await.unwrap();
        }
    }
}
