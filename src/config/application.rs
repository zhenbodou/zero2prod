use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

impl ApplicationSettings {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn application_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
