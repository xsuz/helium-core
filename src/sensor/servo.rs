use crate::sensor::Sensor;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct ServoData{
    pub id: u8,
    pub status:u8,
    pub timestamp: u32,
    pub rudder:f32,
    pub elevator:f32,
    pub voltage:f32,
    pub current_rudder:f32,
    pub current_elevator:f32,
    pub trim:f32,
    pub position_rudder:f32,
    pub position_elevator:f32,
    pub temperature_rudder:f32,
    pub temperature_elevator:f32,
}

impl Sensor for ServoData{
    fn parse(data: &Vec<u8>) -> Self{
        ServoData{
            id: data[0],
            status: data[1],
            timestamp: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            rudder: f32::from_be_bytes(data[8..12].try_into().unwrap()),
            elevator: f32::from_be_bytes(data[12..16].try_into().unwrap()),
            voltage: f32::from_be_bytes(data[16..20].try_into().unwrap()),
            current_rudder: f32::from_be_bytes(data[20..24].try_into().unwrap()),
            current_elevator: f32::from_be_bytes(data[24..28].try_into().unwrap()),
            trim: f32::from_be_bytes(data[28..32].try_into().unwrap()),
            position_rudder: f32::from_be_bytes(data[32..36].try_into().unwrap()),
            position_elevator: f32::from_be_bytes(data[36..40].try_into().unwrap()),
            temperature_rudder: f32::from_be_bytes(data[40..44].try_into().unwrap()),
            temperature_elevator: f32::from_be_bytes(data[44..48].try_into().unwrap()),
        }
    }
    fn get_size() -> usize {
        48
    }
}