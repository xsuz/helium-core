use crate::sensor::Sensor;

#[derive(Debug, Clone, Copy)]
pub struct VaneData{
    pub id: u8,
    pub timestamp: u32,
    pub angle: f32,
}

impl Sensor for VaneData{
    fn parse(data: &Vec<u8>) -> Self{
        VaneData{
            id: data[0],
            timestamp: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            angle: f32::from_be_bytes(data[8..12].try_into().unwrap()),
        }
    }
    fn get_size() -> usize {
        12
    }
}