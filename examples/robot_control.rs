use eye_in_desk_client::EyeInDesk;

// const TRANSLATION_X: u32 = 10;
// const TRANSLATION_Y: u32 = 10;
// const TRANSLATION_Z: u32 = 10;

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    let mut eid1 = eid.clone();

    tokio::spawn(async move {
        let state = eid1.get_real_robot_state().await.unwrap();
        eid1.update_virtual_robot(state.joints).await.unwrap();
    });

    loop {
        let arucos = eid.get_arucos().await.unwrap();
        if arucos.iter().any(|a|a.id == 10) {
            println!("Button pressed");
            let mut t = eid.get_real_robot_state().await.unwrap().transform;
            t.translation.z += 0.1;
            eid.set_real_robot_target(t).await.unwrap();
        }
    }
}