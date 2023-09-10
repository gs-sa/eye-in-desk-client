use eye_in_desk_client::{EyeInDesk, Text};

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    eid.clear_and_draw().await.unwrap();
    eid.place_texts(
        vec![
            Text {
                text: "SUSTech / BionicDL".into(),
                x: 50.,
                y: 200.,
                size: 7.
            }
        ]
    ).await.unwrap();
    eid.clear_and_draw().await.unwrap();
}