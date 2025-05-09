use crate::sensor;
use byteorder::{ByteOrder, BigEndian};

fn convert<T>(data: &mut Vec<(T,i64)>,decoded: &[u8],timestamp:Option<i64>)
where T: sensor::Data+Copy+Clone{
    if decoded.len() < T::get_size() {
        return;
    }
    if let Some(ts) = timestamp {
        data.push((T::parse(&decoded[0..T::get_size()].to_vec()),ts));
    } else {
        data.push((T::parse(&decoded[8..T::get_size()+8].to_vec()),BigEndian::read_i64(&decoded[0..8])));
    }
}

pub struct DataBase {
    pub barometer: Vec<(sensor::BarometerData, i64)>,
    pub gps: Vec<(sensor::GPSData, i64)>,
    pub imu: Vec<(sensor::IMUData, i64)>,
    pub pitot: Vec<(sensor::PitotData, i64)>,
    pub servo: Vec<(sensor::ServoData, i64)>,
    pub tachometer: Vec<(sensor::TachometerData, i64)>,
    pub ultra_sonic: Vec<(sensor::UltraSonicData, i64)>,
    pub vane: Vec<(sensor::VaneData, i64)>,
}

impl DataBase{
    pub fn new()->Self{
        Self {
            barometer: Vec::new(),
            gps: Vec::new(),
            imu: Vec::new(),
            pitot: Vec::new(),
            servo: Vec::new(),
            tachometer: Vec::new(),
            ultra_sonic: Vec::new(),
            vane: Vec::new()
        }
    }
    pub fn update(&mut self, decoded: &Vec<u8>,timestamp:Option<i64>) {
        match decoded[0] & 0xF0 {
            0x10 => convert(&mut self.servo, decoded, timestamp),
            0x20 => convert(&mut self.tachometer, decoded, timestamp),
            0x30 => convert(&mut self.pitot, decoded, timestamp),
            0x40 => convert(&mut self.imu, decoded, timestamp),
            0x50 => convert(&mut self.ultra_sonic, decoded, timestamp),
            0x60 => convert(&mut self.gps, decoded, timestamp),
            0x70 => convert(&mut self.vane, decoded, timestamp),
            0x90 => convert(&mut self.barometer, decoded, timestamp),
            _ => {}
        }
    }
}
