use crate::sensor::Sensor;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct UltraSonicData{
    pub id: u8,
    pub timestamp: u32,
    pub altitude:f32,
    pub temperature:f32,
}

impl Sensor for UltraSonicData{
    fn parse(data: &Vec<u8>) -> Self{
        UltraSonicData{
            id: data[0],
            timestamp: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            altitude: f32::from_be_bytes(data[8..12].try_into().unwrap()),
            temperature: f32::from_be_bytes(data[12..16].try_into().unwrap()),
        }
    }
    fn get_size() -> usize {
        16
    }
}