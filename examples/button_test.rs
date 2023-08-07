use eye_in_desk_client::EyeInDesk;
use eye_in_desk_client::Circle;

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    // let h = Homograph::default();
    eid.clear_and_draw().await.unwrap();
    let init_state = eid.get_real_robot_state().await.unwrap();

    let mut eid1 = eid.clone();
    tokio::spawn(async move {
        let state = eid1.get_real_robot_state().await.unwrap();
        eid1.update_virtual_robot(state.joints).await.unwrap();
    });

    loop {
        let arucos = eid.get_arucos_desktop().await.unwrap();
        if let Some(button) = arucos.iter().find(|a|a.id == 10) {
            println!("Button pressed");
            eid.place_circles(vec![Circle{
                x: button.position.x as f32,
                y: button.position.y as f32,
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