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
        loop {
            let state = eid1.get_real_robot_state().await.unwrap();
            eid1.update_virtual_robot(state.joints).await.unwrap();
        }
    });

    let mut press_filter = (0, 10);
    loop {
        let arucos = eid.get_arucos_desktop().await.unwrap();
        if let Some(_button) = arucos.iter().find(|a|a.id == 10) {
            if press_filter.0 < 10 {press_filter.0 += 1}
            if press_filter.1 > 0 {press_filter.1 -= 1}
            // let p = h.cam_to_projector(Position{x: button.x as f64, y: button.y  as f64});
            eid.place_circles(vec![Circle{
                x: 0. ,
                y: 0. ,
                radius: 200.,
            }]).await.unwrap();
        } else {
            if press_filter.1 < 10 {press_filter.1 += 1}
            if press_filter.0 > 0 {press_filter.0 -= 1}
        }
        println!("{:?}", press_filter);
        if press_filter.0 >= 5 {
            let mut t = init_state.transform;
            t.translation.z += 0.1;
            eid.set_real_robot_target(t).await.unwrap();
        } else {
            eid.set_real_robot_target(init_state.transform).await.unwrap();
        }
        eid.clear_and_draw().await.unwrap();
    }
}