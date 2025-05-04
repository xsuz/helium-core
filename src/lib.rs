pub mod cobs;
pub mod sensor;

use byteorder::{ByteOrder, BigEndian};

use sensor::Data;

pub fn parse_data<T>(data: &mut Vec<(T,i64)>,decoded: &[u8],timestamp:Option<i64>)
where T: Data+Copy+Clone{
    if decoded.len() < T::get_size() {
        return;
    }
    if let Some(ts) = timestamp {
        data.push((T::parse(&decoded[0..T::get_size()].to_vec()),ts));
    } else {
        data.push((T::parse(&decoded[8..T::get_size()+8].to_vec()),BigEndian::read_i64(&decoded[0..8])));
    }
}