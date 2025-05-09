pub mod imu;
pub mod gps;
pub mod barometer;
pub mod pitot;
pub mod servo;
pub mod tachometer;
pub mod ultra_sonic;
pub mod vane;

pub use imu::IMUData;
pub use gps::GPSData;
pub use barometer::BarometerData;
pub use pitot::PitotData;
pub use servo::ServoData;
pub use tachometer::TachometerData;
pub use ultra_sonic::UltraSonicData;
pub use vane::VaneData;
pub trait Data{
    fn parse(data: &Vec<u8>) -> Self;
    fn get_size() -> usize;
}