fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "../eye-in-desk/protos/web.proto",
                "../eye-in-desk/protos/camera.proto",
                "../eye-in-desk/protos/projector.proto",
                "../eye-in-desk/protos/robot.proto",
            ],
            &["../eye-in-desk/protos/"],
        )?;
    Ok(())
}
