use dashmap::DashMap;

pub struct AppState {
    pub merkle_map: DashMap<String, u32>,
}