use crate::sensor::Data;
use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct TachometerData{
    pub id: u8,
    pub timestamp: u32,
    pub cadence:f32,
    pub strain:u32
}

impl Data for TachometerData{
    fn parse(data: &Vec<u8>) -> Self{
        TachometerData{
            id: data[0],
            timestamp: BigEndian::read_u32(&data[4..8]),
            cadence: BigEndian::read_f32(&data[8..12]),
            strain: BigEndian::read_u32(&data[12..16]),
        }
    }
    fn get_size() -> usize {
        16
    }
}