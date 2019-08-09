use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LogConfig {
    pub debug: bool
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub min_size: usize,
    pub max_size: usize,
    pub max_rooms: usize,
    pub tile_size: usize
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PanelConfig {
    pub bottom: usize,
    pub right: usize,
    pub bottom_lines: usize,
    pub font_size: usize
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FovConfig {
    pub radius: usize,
    pub distance: usize
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BlobsConfig {
    pub log: LogConfig,
    pub map: MapConfig,
    pub panel: PanelConfig,
    pub fov: FovConfig,
}
