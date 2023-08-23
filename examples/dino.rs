use eye_in_desk_client::EyeInDesk;
use enigo::*;

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    let mut enigo = Enigo::new();
    loop {
        let arucos = eid.get_arucos().await.unwrap();
        if arucos.iter().any(|a|a.id == 10) {
            println!("Button pressed");
            enigo.key_click(Key::Space);
        }
    }
}
