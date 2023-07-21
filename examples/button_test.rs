use eye_in_desk_client::EyeInDesk;
use eye_in_desk_client::Circle;
use eye_in_desk_client::Homograph;
use eye_in_desk_client::Position;
#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    let h = Homograph::default();
    eid.clear_and_draw().await.unwrap();
    let init_state = eid.get_real_robot_state().await.unwrap();

    let mut eid1 = eid.clone();
    tokio::spawn(async move {
        let state = eid1.get_real_robot_state().await.unwrap();
        eid1.update_virtual_robot(state.joints).await.unwrap();
    });

    loop {
        let arucos = eid.get_arucos().await.unwrap();
        if let Some(button) = arucos.iter().find(|a|a.id == 10) {
            println!("Button pressed");
            let p = h.cam_to_projector(Position{x: button.x as f64, y: button.y  as f64});
            eid.place_circles(vec![Circle{
                x: p.x as f32,
                y: p.y as f32,
                radius: 200.,
            }]).await.unwrap();
            let mut t = init_state.transform;
            t.translation.z += 0.1;
            eid.set_real_robot_target(t).await.unwrap();
        } else {
            eid.set_real_robot_target(init_state.transform).await.unwrap();
        }
        eid.clear_and_draw().await.unwrap();
    }
    
}