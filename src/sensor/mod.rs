pub mod barometer;
pub mod gps;
pub mod imu;
pub mod pitot;
pub mod servo;
pub mod tachometer;
pub mod ultra_sonic;
pub mod vane;

pub use barometer::BarometerData;
pub use gps::GPSData;
pub use imu::IMUData;
pub use pitot::PitotData;
pub use servo::ServoData;
pub use tachometer::TachometerData;
pub use ultra_sonic::UltraSonicData;
pub use vane::VaneData;

pub trait Sensor {
    fn parse(data: &Vec<u8>) -> Self;
    fn get_size() -> usize;
}