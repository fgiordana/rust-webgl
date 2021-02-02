
#[derive(Clone)]
pub struct App {
    time: f64
}

impl App {
    pub fn new() -> Self {
        App {
            time: 0.0
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
    }
}
