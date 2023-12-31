use camera_rpc::{
    camera_service_client::CameraServiceClient, ArucoPosition, GetArucosPositionRequest,
};
use config::EIDConfig;
use projector_rpc::{
    projector_service_client::ProjectorServiceClient, DrawArucosRequest, DrawCirclesRequest,
    DrawLinesRequest, DrawRectanglesRequest, DrawRequest, DrawTextsRequest, GetDrawableSizeRequest,
    Line, Rectangle,
};

pub use projector_rpc::{Aruco, Circle, Text};

use robot_rpc::{
    robot_service_client::RobotServiceClient, GetRobotInfoRequest, SetRobotModeRequest,
    SetRobotTargetRequest,
};
pub use sim_rpc::Object;
use sim_rpc::{
    web_service_client::WebServiceClient, CameraControlRequest, ShowObjectsRequest,
    UpdateRobotRequest,
};

use nalgebra::{Isometry3, Matrix4, Rotation3, Vector3};
use tonic::{codegen::StdError, transport::Channel, Status};

mod sim_rpc {
    tonic::include_proto!("web");
}

mod projector_rpc {
    tonic::include_proto!("projector");
}

mod camera_rpc {
    tonic::include_proto!("camera");
}

mod robot_rpc {
    tonic::include_proto!("robot");
}

mod config;
pub use config::Position;

mod calibration;

fn array_to_isometry(array: &[f64]) -> Isometry3<f64> {
    let rot = Rotation3::from_matrix(
        &Matrix4::from_column_slice(array)
            .remove_column(3)
            .remove_row(3),
    );
    Isometry3::from_parts(
        Vector3::new(array[12], array[13], array[14]).into(),
        rot.into(),
    )
}

pub struct RobotState {
    pub joints: Vec<f64>,
    pub transform: Isometry3<f64>,
}

static PROJ_PORT: u16 = 50051;
static SIM_PORT: u16 = 50052;
static CAM_PORT: u16 = 50053;
static ROBOT_PORT: u16 = 50054;

#[derive(Clone)]
pub struct EyeInDesk {
    cam_client: CameraServiceClient<Channel>,
    proj_client: ProjectorServiceClient<Channel>,
    sim_client: WebServiceClient<Channel>,
    robot_client: Option<RobotServiceClient<Channel>>,
    pub config: EIDConfig,
}

pub struct ArucoDesktopPosition {
    pub id: i32,
    pub position: Position,
    pub rot: f32,
}

pub enum RobotMode {
    Target,
    Drag,
}

impl EyeInDesk {
    /// connect with defalut address
    pub async fn default_connect() -> Self {
        let proj_addr = format!("http://127.0.0.1:{PROJ_PORT}");
        let sim_addr = format!("http://127.0.0.1:{SIM_PORT}");
        let cam_addr = format!("http://127.0.0.1:{CAM_PORT}");
        let robot_addr = format!("http://127.0.0.1:{ROBOT_PORT}");
        EyeInDesk::connect(cam_addr, proj_addr, sim_addr, robot_addr).await
    }

    pub async fn connect<A>(cam_addr: A, proj_addr: A, sim_addr: A, robot_addr: A) -> Self
    where
        A: TryInto<tonic::transport::Endpoint>,
        A::Error: Into<StdError>,
    {
        let cam_client: CameraServiceClient<Channel> =
            CameraServiceClient::connect(cam_addr).await.unwrap();
        let proj_client = ProjectorServiceClient::connect(proj_addr).await.unwrap();
        let sim_client = WebServiceClient::connect(sim_addr).await.unwrap();
        let robot_client = match RobotServiceClient::connect(robot_addr).await {
            Ok(c) => Some(c),
            Err(_) => None,
        };
        let config = EIDConfig::default();
        EyeInDesk {
            cam_client,
            proj_client,
            sim_client,
            robot_client,
            config,
        }
    }

    pub async fn get_arucos(&mut self) -> Result<Vec<ArucoPosition>, Status> {
        self.cam_client
            .get_arucos_position(GetArucosPositionRequest {})
            .await
            .map(|resp| resp.into_inner().arucos)
    }

    pub async fn get_arucos_desktop(&mut self) -> Result<Vec<ArucoDesktopPosition>, Status> {
        self.get_arucos().await.map(|v| {
            v.into_iter()
                .map(|a| ArucoDesktopPosition {
                    id: a.id,
                    position: self.config.cam_to_projector(Position {
                        x: a.x as f64,
                        y: a.y as f64,
                    }),
                    rot: a.rot,
                })
                .collect()
        })
    }

    pub async fn get_drawable_size(&mut self) -> Result<(f64, f64), Status> {
        self.proj_client
            .get_drawable_size(GetDrawableSizeRequest {})
            .await
            .map(|resp| {
                let resp = resp.into_inner();
                (resp.width, resp.height)
            })
    }

    pub async fn place_arucos(&mut self, arucos: Vec<Aruco>) -> Result<(), Status> {
        self.proj_client
            .draw_arucos(DrawArucosRequest { markers: arucos })
            .await
            .map(|_| ())
    }

    pub async fn place_texts(&mut self, texts: Vec<Text>) -> Result<(), Status> {
        self.proj_client
            .draw_texts(DrawTextsRequest { texts })
            .await
            .map(|_| ())
    }

    pub async fn place_circles(&mut self, circles: Vec<Circle>) -> Result<(), Status> {
        self.proj_client
            .draw_circles(DrawCirclesRequest { circles })
            .await
            .map(|_| ())
    }

    pub async fn place_lines(&mut self, lines: Vec<Line>) -> Result<(), Status> {
        self.proj_client
            .draw_lines(DrawLinesRequest { lines })
            .await
            .map(|_| ())
    }

    pub async fn place_rects(&mut self, rectangles: Vec<Rectangle>) -> Result<(), Status> {
        self.proj_client
            .draw_rectangles(DrawRectanglesRequest { rectangles })
            .await
            .map(|_| ())
    }

    pub async fn clear_and_draw(&mut self) -> Result<(), Status> {
        self.proj_client.draw(DrawRequest {}).await.map(|_| ())
    }

    pub async fn update_virtual_objects(&mut self, objects: Vec<Object>) -> Result<(), Status> {
        self.sim_client
            .show_objects(ShowObjectsRequest { objects })
            .await
            .map(|_| ())
    }

    pub async fn update_virtual_robot(&mut self, robot: Vec<f64>) -> Result<(), Status> {
        self.sim_client
            .update_robot(UpdateRobotRequest { robot })
            .await
            .map(|_| ())
    }

    pub async fn camera_control(&mut self, rotate_left: f32, rotate_up: f32) -> Result<(), Status> {
        self.sim_client
            .camera_control(CameraControlRequest {
                rotate_left,
                rotate_up,
            })
            .await
            .map(|_| ())
    }

    pub async fn get_real_robot_state(&mut self) -> Result<RobotState, Status> {
        self.robot_client
            .clone()
            .ok_or(Status::data_loss("no robot connection"))?
            .get_robot_info(GetRobotInfoRequest {})
            .await
            .map(|resp| {
                let resp = resp.into_inner();
                RobotState {
                    joints: resp.joints,
                    transform: array_to_isometry(&resp.t),
                }
            })
    }

    pub async fn set_real_robot_target(&mut self, transfrom: Isometry3<f64>) -> Result<(), Status> {
        self.robot_client
            .clone()
            .ok_or(Status::data_loss("no robot connection"))?
            .set_robot_target(SetRobotTargetRequest {
                t: transfrom.to_matrix().as_slice().to_vec(),
            })
            .await
            .map(|_| ())
    }

    pub async fn set_robot_mode(&mut self, mode: RobotMode) -> Result<(), Status> {
        let mode = match mode {
            RobotMode::Target => 0,
            RobotMode::Drag => 1,
        };
        self.robot_client
            .clone()
            .ok_or(Status::data_loss("no robot connection"))?
            .set_robot_mode(SetRobotModeRequest { mode })
            .await
            .map(|_| ())
    }
}

#[tokio::test]
async fn eye_in_desk_connect() {
    EyeInDesk::default_connect().await;
}

#[tokio::test]
async fn eye_in_desk_get_aruco() {
    use std::result::Result::Ok;
    let mut eid = EyeInDesk::default_connect().await;
    while let Ok(arucos) = eid.get_arucos().await {
        if !arucos.is_empty() {
            println!("{:?}", arucos);
            break;
        }
    }
}

#[tokio::test]
async fn eye_in_desk_get_drawable_size() {
    let mut eid = EyeInDesk::default_connect().await;
    let size = eid.get_drawable_size().await.unwrap();
    println!("{:?}", size);
}

#[tokio::test]
async fn eye_in_desk_draw() {
    let mut eid = EyeInDesk::default_connect().await;
    eid.place_arucos(vec![Aruco {
        x: 100.,
        y: 100.,
        size: 200.,
    }])
    .await
    .unwrap();
    // eid.place_texts(vec![Text {
    //     x: 960.0,
    //     y: 200.0,
    //     text: "Hello World".to_string(),
    //     size: 5.0,
    // }])
    // .await
    // .unwrap();
    // eid.place_circles(vec![Circle {
    //     x: 0.0,
    //     y: 0.0,
    //     radius: 200.0,
    // }])
    // .await
    // .unwrap();
    eid.place_lines(vec![Line {
        x1: 0.,
        y1: 0.,
        x2: 500.,
        y2: 500.,
        width: 20.,
    }])
    .await
    .unwrap();

    eid.place_rects(vec![Rectangle {
        x: 50.,
        y: 50.,
        width: 300.,
        height: 300.,
        line_width: 20.,
        fill: false,
    }])
    .await
    .unwrap();
    eid.clear_and_draw().await.unwrap();
}

#[tokio::test]
async fn eye_in_desk_update_virtaul_objects() {
    let mut eid = EyeInDesk::default_connect().await;
    let objects = vec![Object {
        x: 100.0,
        y: 0.0,
        id: 0,
        z: 0.0,
        rot: 0.0,
        scale: 1.0,
    }];
    eid.update_virtual_objects(objects).await.unwrap();
}

#[tokio::test]
async fn eye_in_desk_update_virtaul_robot() {
    use std::f64::consts::PI;
    let mut eid = EyeInDesk::default_connect().await;
    let joints = vec![0., -PI / 4., 0., -3. * PI / 4., 0., PI / 2., PI / 4.];
    eid.update_virtual_robot(joints).await.unwrap();
}

#[tokio::test]
async fn eye_in_desk_camera_control() {
    let mut eid = EyeInDesk::default_connect().await;
    eid.camera_control(10., 0.).await.unwrap();
}

#[tokio::test]
async fn eye_in_desk_get_real_robot_state() {
    let mut eid = EyeInDesk::default_connect().await;
    let state = eid.get_real_robot_state().await.unwrap();
    println!("{:?}", state.joints);
    println!("{}", state.transform);
}

#[tokio::test]
async fn eye_in_desk_set_real_robot_target() {
    let mut eid = EyeInDesk::default_connect().await;
    let state = eid.get_real_robot_state().await.unwrap();
    let mut t = state.transform;
    t.translation.z += 0.1;
    eid.set_real_robot_target(t).await.unwrap();
}

#[tokio::test]
async fn eye_in_desk_set_robot_mode() {
    let mut eid = EyeInDesk::default_connect().await;
    eid.set_robot_mode(RobotMode::Target).await.unwrap();
}
