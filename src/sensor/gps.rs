use crate::sensor::Data;
use byteorder::{BigEndian, ByteOrder};

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

impl Data for GPSData{
    fn parse(data: &Vec<u8>) -> Self{
        GPSData{
            id: BigEndian::read_u32(&data[0..4]),
            timestamp: BigEndian::read_u32(&data[4..8]),
            latitude: BigEndian::read_f64(&data[8..16]),
            longitude: BigEndian::read_f64(&data[16..24]),
            altitude: BigEndian::read_f32(&data[24..28]),
            east_velocity: BigEndian::read_f32(&data[28..32]),
            north_velocity: BigEndian::read_f32(&data[32..36]),
            hdop: BigEndian::read_f32(&data[36..40]),
        }
    }
    fn get_size() -> usize {
        40
    }
}