use eye_in_desk_client::{EyeInDesk, Circle};
use enigo::{Enigo, KeyboardControllable};
 
#[tokio::main]
async fn main() {
    let mut enigo = Enigo::new();
    let mut eid = EyeInDesk::default_connect().await;
    eid.clear_and_draw().await.unwrap();
    loop {
        let arucos = eid.get_arucos_desktop().await.unwrap();
        if let Some(aruco) = arucos.iter().find(|a|a.id == 10) {
            enigo.key_down(enigo::Key::Space);
            eid.place_circles(vec![
                Circle {
                    x: aruco.position.x as f32,
                    y: aruco.position.y as f32,
                    radius: 100.,
                    fill: false,
                }
            ]).await.unwrap();    
        } else {
            enigo.key_up(enigo::Key::Space);
        }
        eid.clear_and_draw().await.unwrap();
    }
}
