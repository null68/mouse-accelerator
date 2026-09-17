#[derive(Debug, Clone, Copy)]
pub struct AccelParams {
    pub sens: f64,
    pub accel: f64,
    pub power: f64,
    pub cap: f64,
}

impl Default for AccelParams {
    fn default() -> Self {
        Self { sens: 1.0, accel: 0.03, power: 2.0, cap: 3.0 }
    }
}

pub fn apply(dx: i32, dy: i32, p: &AccelParams) -> (i32, i32) {
    if dx == 0 && dy == 0 {
        return (0, 0);
    }
    let speed = ((dx * dx + dy * dy) as f64).sqrt();
    let multiplier = (p.sens + p.accel * speed.powf(p.power)).min(p.cap);
    (
        (dx as f64 * multiplier).round() as i32,
        (dy as f64 * multiplier).round() as i32,
    )
}
