use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub tile_size: usize
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BlobsConfig {
    pub map: MapConfig
}
