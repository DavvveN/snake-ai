use rand::Rng;

pub struct Brain {
    pub weights: Vec<f32>, // len == 36 (3 x 12) - single peceptron
}

impl Brain {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let weigths = (0..(3 * 12)).map(|_| rng.random_range(1.0..1.0)).collect();
        Self { weights: weigths }
    }
}
