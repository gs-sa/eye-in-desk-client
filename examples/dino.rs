use enigo::{Enigo, KeyboardControllable};
use eye_in_desk_client::EyeInDesk;

#[tokio::main]
async fn main() {
    let mut enigo = Enigo::new();
    let mut eid = EyeInDesk::default_connect().await;
    eid.clear_and_draw().await.unwrap();
    loop {
        let arucos = eid.get_arucos().await.unwrap();
        if arucos.iter().any(|a| a.id == 10) {
            enigo.key_down(enigo::Key::Space);
        } else {
            enigo.key_up(enigo::Key::Space);
        }
    }
}
