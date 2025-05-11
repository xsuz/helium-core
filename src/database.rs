use crate::{sensor, series::Series};


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct DataBase {
    pub barometer: Series<sensor::BarometerData>,
    pub gps: Series<sensor::GPSData>,
    pub imu: Series<sensor::IMUData>,
    pub pitot: Series<sensor::PitotData>,
    pub servo: Series<sensor::ServoData>,
    pub tachometer: Series<sensor::TachometerData>,
    pub ultra_sonic: Series<sensor::UltraSonicData>,
    pub vane: Series<sensor::VaneData>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            barometer: Series::new(),
            gps: Series::new(),
            imu: Series::new(),
            pitot: Series::new(),
            servo: Series::new(),
            tachometer: Series::new(),
            ultra_sonic: Series::new(),
            vane: Series::new(),
        }
    }
    pub fn update(&mut self, decoded: &Vec<u8>, timestamp: Option<i64>) {
        match if timestamp.is_none() {
            decoded[8]
        } else {
            decoded[0]
        } & 0xF0
        {
            0x10 => self.servo.parse(decoded, timestamp),
            0x20 => self.tachometer.parse(decoded, timestamp),
            0x30 => self.pitot.parse(decoded, timestamp),
            0x40 => self.imu.parse(decoded, timestamp),
            0x50 => self.ultra_sonic.parse(decoded, timestamp),
            0x60 => self.gps.parse(decoded, timestamp),
            0x70 => self.vane.parse(decoded, timestamp),
            0x90 => self.barometer.parse(decoded, timestamp),
            _ => {}
        }
    }
}
