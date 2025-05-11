use crate::sensor::Sensor;


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Series<T: Sensor> {
    pub data: Vec<(T, i64)>,
}

impl<T: Sensor> Series<T> {
    pub fn new() -> Self {
        Series { data: Vec::new() }
    }
    pub fn parse(&mut self, decoded: &[u8], timestamp: Option<i64>) {
        if decoded.len() < T::get_size() {
            return;
        }
        if let Some(ts) = timestamp {
            self.data
                .push((T::parse(&decoded[0..T::get_size()].to_vec()), ts));
        } else {
            self.data.push((
                T::parse(&decoded[8..T::get_size() + 8].to_vec()),
                i64::from_be_bytes(decoded[0..8].try_into().unwrap()),
            ));
        }
    }
}