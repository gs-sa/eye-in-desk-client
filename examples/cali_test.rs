use eye_in_desk_client::{EyeInDesk, Circle};

static PRE_CALI:bool = false;
static EXP_INDEX:i32 = 0;

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    if PRE_CALI {eid.calibration().await.unwrap()}

    // let res = eid.config.homography.data * Vector3::new(1384.5, 560., 1.0);
    // println!("{}", eid.config.homography.data);
    // println!("{res:?} {} {}", res.x/res.z, res.y/res.z);
    
    // expe
    match EXP_INDEX {
        0 => {
            aruco_track(&mut eid).await;
        }
        1 => {
            // heat_map(&mut eid).await;
        }
        _ => {}
    }
}

async fn aruco_track(eid: &mut EyeInDesk) {
    eid.clear_and_draw().await.unwrap();
    loop {
        let arucos = eid.get_arucos_desktop().await.unwrap();
        for aruco in arucos {
            eid.place_circles(vec![
                Circle {
                    x: aruco.position.x as f32,
                    y: aruco.position.y as f32,
                    radius: 150.
                }
            ]).await.unwrap();
        }
        eid.clear_and_draw().await.unwrap();
    }
}

// async fn heat_map(eid: &mut EyeInDesk) {
//     eid.clear_and_draw().await.unwrap();
// }