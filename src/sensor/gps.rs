use crate::sensor::Sensor;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct GPSData{
    pub id: u32,
    pub timestamp: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub east_velocity: f32,
    pub north_velocity: f32,
    pub hdop: f32
}

impl Sensor for GPSData{
    fn parse(data: &Vec<u8>) -> Self{
        GPSData{
            id: u32::from_be_bytes(data[0..4].try_into().unwrap()),
            timestamp: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            latitude: f64::from_be_bytes(data[8..16].try_into().unwrap()),
            longitude: f64::from_be_bytes(data[16..24].try_into().unwrap()),
            altitude: f32::from_be_bytes(data[24..28].try_into().unwrap()),
            east_velocity: f32::from_be_bytes(data[28..32].try_into().unwrap()),
            north_velocity: f32::from_be_bytes(data[32..36].try_into().unwrap()),
            hdop: f32::from_be_bytes(data[36..40].try_into().unwrap()),
        }
    }
    fn get_size() -> usize {
        40
    }
}