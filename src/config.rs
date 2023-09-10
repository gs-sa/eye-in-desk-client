use nalgebra::{Matrix3, Vector3};
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Debug)]
pub struct EIDConfigJSON {
    pub homography: Vec<f64>,
}

impl EIDConfigJSON {
    pub fn read_config_file(path: &str) -> anyhow::Result<Self> {
        let config_file = std::fs::read_to_string(path)?;
        let a = serde_json::from_str(&config_file)?;
        Ok(a)
    }

    pub fn create_config_file(path: &str) -> anyhow::Result<Self> {
        let config = EIDConfigJSON {
            homography: vec![
                1.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, //
                0.0, 0.0, 1.0, //
            ],
        };
        let config_file = serde_json::to_string_pretty(&config)?;
        std::fs::write(path, config_file)?;
        Ok(config)
    }

    pub fn write_defalut(&self)  -> anyhow::Result<()> {
        let config_file = serde_json::to_string_pretty(self)?;
        std::fs::write("config.json", config_file)?;
        Ok(())
    }

    pub fn defalut() -> Self {
        let Ok(config) = Self::read_config_file("config.json") else {
            if let Ok(config) = Self::create_config_file("config.json") {
                return config
            } else {
                panic!("Failed to create config file")
            }
        };
        config
    }
}

#[derive(Clone)]
pub struct EIDConfig {
    pub homography: Homography,
}

impl EIDConfig {
    fn from_json(json: EIDConfigJSON) -> Self {
        let homography = Matrix3::from_column_slice(&json.homography);
        let homography = Homography { data: homography };
        EIDConfig { homography }
    }

    pub fn to_json(&self) -> EIDConfigJSON {
        EIDConfigJSON {
            homography: self.homography.data.as_slice().to_vec()
        }
    }

    pub fn from_matrix(m: Matrix3<f64>) -> Self {
        EIDConfig {
            homography: Homography {
                data: m
            }
        }
    }

    pub fn default() -> Self {
        let json = EIDConfigJSON::defalut();
        Self::from_json(json)
    }

    pub fn cam_to_projector(&self, cam_pos: Position) -> Position {
        self.homography.cam_to_projector(cam_pos)
    }
}

#[derive(Clone)]
pub struct Homography {
    pub data: Matrix3<f64>,
}

impl Homography {
    pub fn cam_to_projector(&self, cam_pos: Position) -> Position {
        let cam = Vector3::new(cam_pos.x, cam_pos.y, 1.);
        let proj = self.data * cam;
        let proj = proj.scale(proj.z.recip());
        Position {
            x: proj.x,
            y: proj.y,
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}