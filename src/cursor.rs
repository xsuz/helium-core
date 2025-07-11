use crate::sensor::Sensor;
use crate::series::Series;

/// A struct representing a cursor that tracks a position within a time series data set.
/// The cursor is defined by a time range and an index within the data series.
/// This cursor can be used to navigate through the data series, allowing for efficient querying and visualization of time-based data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    /// The time range in milliseconds that the cursor covers.
    pub range: i64,
    /// The index in the data series where the cursor is currently positioned.
    pub index: usize
}

impl Cursor {
    /// Creates a new Cursor with the specified range and initial index.
    /// # Arguments
    /// * `range`: The time range in milliseconds that the cursor should cover.
    /// * `index`: The initial index in the data series.
    pub fn new(range: i64, index: usize) -> Self {
        Cursor { range, index }
    }
    /// Updates the cursor's index based on the provided data series and an optional timestamp.
    /// # Arguments
    /// * `db`: A reference to the data series of type `Series<T>`, where `T` implements the `Sensor` trait.
    /// * `to`: An optional timestamp in milliseconds. If `None`, the cursor will use the last timestamp in the data series.
    pub fn update<T: Sensor>(&mut self, series: &Series<T>, to: Option<i64>) {
        if series.data.len() < 2 {
            return;
        }
        let to = if let Some(to) = to {
            to
        } else {
            if let Some(last) = series.data.last() {
                last.1
            } else {
                return;
            }
        };
        
        if series.data[self.index].1 + self.range > to {
            self.index -= series.data[..self.index]
                .iter()
                .rev()
                .position(|x| x.1 + self.range <= to)
                .unwrap_or(self.index);
        }else if self.index < series.data.len() - 1 {
            if series.data[self.index + 1].1 + self.range <= to {
                self.index += series.data[self.index+1..]
                    .iter()
                    .position(|x| x.1 + self.range > to)
                    .unwrap_or(series.data.len() - self.index - 1);
            }
        }
    }
}
