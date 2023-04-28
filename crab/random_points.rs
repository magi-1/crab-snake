use pyo3::prelude::*;
use rand::distributions::{Distribution, Normal};
use rayon::prelude::*;
use std::time::Instant;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn move_point(&mut self, normal: &Normal) {
        self.x += normal.sample(&mut rand::thread_rng());
        self.y += normal.sample(&mut rand::thread_rng());
    }
}

#[pyfunction]
pub fn run(num_points: usize, num_steps: usize) {
    let start_time = Instant::now();
    let normal = Normal::new(0.0, 1.0);
    let mut points = Vec::with_capacity(num_points);

    for _ in 0..num_points {
        points.push(Point::new());
    }

    for _ in 0..num_steps {
        points.par_iter_mut().for_each(|p| p.move_point(&normal));
    }

    println!(
        "(rust) Executed in {:.2} seconds",
        start_time.elapsed().as_secs_f64()
    );
}
