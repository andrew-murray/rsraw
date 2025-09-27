use rsraw_sys as sys;

#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GpsInfo {
    pub latitude: [f32; 3],
    pub longitude: [f32; 3],
    pub gpstimestamp: [f32; 3],
    pub altitude: f32,
}

impl From<sys::libraw_gps_info_t> for GpsInfo {
    fn from(data: sys::libraw_gps_info_t) -> Self {
        Self {
            latitude: data.latitude,
            longitude: data.longitude,
            gpstimestamp: data.gpstimestamp,
            altitude: data.altitude,
        }
    }
}
