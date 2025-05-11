use crate::sensor::Sensor;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PitotData{
    pub id: u8,
    pub timestamp: u32,
    pub pressure:f32,
    pub temperature:f32,
    pub velocity:f32
}

impl Sensor for PitotData{
    fn parse(data: &Vec<u8>) -> Self{
        PitotData{
            id: data[0],
            timestamp: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            pressure: f32::from_be_bytes(data[8..12].try_into().unwrap()),
            temperature: f32::from_be_bytes(data[12..16].try_into().unwrap()),
            velocity: f32::from_be_bytes(data[16..20].try_into().unwrap()),
        }
    }
    fn get_size() -> usize {
        20
    }
}