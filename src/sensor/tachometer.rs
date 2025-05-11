use crate::sensor::Sensor;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct TachometerData{
    pub id: u8,
    pub timestamp: u32,
    pub cadence:f32,
    pub strain:u32
}

impl Sensor for TachometerData{
    fn parse(data: &Vec<u8>) -> Self{
        TachometerData{
            id: data[0],
            timestamp: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            cadence: f32::from_be_bytes(data[8..12].try_into().unwrap()),
            strain: u32::from_be_bytes(data[12..16].try_into().unwrap()),
        }
    }
    fn get_size() -> usize {
        16
    }
}