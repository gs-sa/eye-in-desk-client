use anyhow::Ok;
use eye_in_desk_client::{EyeInDesk, RobotState};

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    eid.clear_and_draw().await.unwrap();
    // drag
    // eid.set_robot_drag_mode().await.unwrap();
    // record
    let mut records = vec![];
    let state = eid.get_real_robot_state().await.unwrap();
    records.push(state);
    // play
    play(&eid, records).await.unwrap();
}

async fn button_press(eid: &EyeInDesk, button: i32) {}

async fn play(eid: &EyeInDesk, records: Vec<RobotState>) -> anyhow::Result<()> {
    Ok(())
}
