pub mod imu;
pub mod gps;

pub use imu::IMUData;
pub use gps::GPSData;

pub trait Data{
    fn parse(data: &Vec<u8>) -> Self;
    fn get_size() -> usize;
}