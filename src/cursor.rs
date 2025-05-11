use crate::sensor::Sensor;
use crate::series::Series;

pub struct Cursor {
    pub range: i64,
    pub index: usize,
    pub to: Option<i64>, // None : to end
}

impl Cursor {
    pub fn new(range: i64, index: usize, to: Option<i64>) -> Self {
        Cursor { range, index, to }
    }
    pub fn update<T: Sensor>(&mut self, db: &Series<T>) {
        let to = if let Some(to) = self.to {
            to
        } else {
            if let Some(last) = db.data.last() {
                last.1
            } else {
                return;
            }
        };
        if db.data[self.index].1 + self.range < to {
            self.index = db.data[self.index..]
                .iter()
                .position(|x| x.1 + self.range >= to)
                .unwrap_or(db.data.len() - 1);
        } else {
            self.index = db.data[..self.index]
                .iter()
                .rev()
                .position(|x| x.1 + self.range <= to)
                .unwrap_or(0);
        }
    }
}
