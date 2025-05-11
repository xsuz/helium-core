use crate::sensor::Sensor;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct IMUData{
    pub id: u32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub offset_quaternion: Option<(f64,f64,f64,f64)>,
}

impl Sensor for IMUData{
    fn parse(data: &Vec<u8>) -> Self{
        IMUData{
            id: u32::from_be_bytes(data[0..4].try_into().unwrap()),
            gyro_x: f32::from_be_bytes(data[4..8].try_into().unwrap()),
            gyro_y: f32::from_be_bytes(data[8..12].try_into().unwrap()),
            gyro_z: f32::from_be_bytes(data[12..16].try_into().unwrap()),
            accel_x: f32::from_be_bytes(data[16..20].try_into().unwrap()),
            accel_y: f32::from_be_bytes(data[20..24].try_into().unwrap()),
            accel_z: f32::from_be_bytes(data[24..28].try_into().unwrap()),
            offset_quaternion: None
        }
    }
    fn get_size() -> usize {
        28
    }
}