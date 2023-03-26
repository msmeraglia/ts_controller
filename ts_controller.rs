use std::time::Instant;

pub struct TimestepController {
    current_time: Instant,
    t: f64,
    dt: f64,
    accumulator: f64,
}

impl TimestepController {
    pub fn new(dt: f64) -> Self {
        let t: f64 = 0.0;
        let current_time = Instant::now();
        let accumulator: f64 = 0.0;

        Self {
            current_time,
            t,
            dt,
            accumulator,
        }
    }

    fn advance_timestep(&mut self) {
        let new_time = Instant::now();
        let mut frame_time = (new_time - self.current_time).as_secs_f64();
        if frame_time > 0.25 {
            frame_time = 0.25
        }
        self.current_time = new_time;
        self.accumulator += frame_time;
    }

    // return value is a blend factor
    // blend factor gives value [0, 1] that can be used to
    // linearly interpolate between physics steps between frame times
    pub fn sync_with_ts<F>(&mut self, mut f: F) -> f64
    where
        F: FnMut(f64, f64),
    {
        self.advance_timestep();
        while self.accumulator >= self.dt {
            f(self.dt, self.t);
            self.t += self.dt;
            self.accumulator -= self.dt;
        }
        self.accumulator / self.dt
    }
}
