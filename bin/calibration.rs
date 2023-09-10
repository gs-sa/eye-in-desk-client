use eye_in_desk_client::EyeInDesk;

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    eid.clear_and_draw().await.unwrap();
    eid.calibration().await.unwrap();
}