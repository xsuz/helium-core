use crate::sensor::Data;
use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
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

impl Data for IMUData{
    fn parse(data: &Vec<u8>) -> Self{
        IMUData{
            id: BigEndian::read_u32(&data[0..4]),
            gyro_x: BigEndian::read_f32(&data[4..8]),
            gyro_y: BigEndian::read_f32(&data[8..12]),
            gyro_z: BigEndian::read_f32(&data[12..16]),
            accel_x: BigEndian::read_f32(&data[16..20]),
            accel_y: BigEndian::read_f32(&data[20..24]),
            accel_z: BigEndian::read_f32(&data[24..28]),
            offset_quaternion: None
        }
    }
    fn get_size() -> usize {
        28
    }
}