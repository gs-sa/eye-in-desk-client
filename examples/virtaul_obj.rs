use eye_in_desk_client::{EyeInDesk, Object};

#[tokio::main]
async fn main() {
    let mut eid = EyeInDesk::default_connect().await;
    loop {
        let arucos = eid.get_arucos().await.unwrap();
        let objects = arucos
            .iter()
            .map(|a| Object {
                id: a.id + 100,
                x: a.x,
                y: a.y,
                z: 0.,
                rot: a.rot,
                scale: 1.,
            })
            .collect::<Vec<_>>();
        eid.update_virtual_objects(objects).await.unwrap();
    }
}
