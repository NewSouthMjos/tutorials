use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub uuid: Uuid,
    pub mac: String,
    pub firmware: String,
}