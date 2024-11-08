#[derive(Debug, Clone)]
pub struct Location {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub demand: u32,
    pub ready_time: u32,
    pub due_date: u32,
    pub service_time: u32,
}

impl Location {
    pub fn distance_to(&self, other: &Location) -> f64 {
        f64::from(u32::pow(other.x.abs_diff(self.x), 2) + u32::pow(other.y.abs_diff(self.y), 2))
            .sqrt()
    }
}
