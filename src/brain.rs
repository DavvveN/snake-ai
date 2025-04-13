use rand::Rng;

#[derive(Debug, Clone)]
pub struct Brain {
    pub weights: Vec<f32>, // len == 36 (3 x 12) - single peceptron
}

impl Brain {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let weigths = (0..(3 * 12)).map(|_| rng.random_range(-1.0..1.0)).collect();
        Self { weights: weigths }
    }

    pub fn decide(&self, state: &Vec<f32>) -> usize {
        let action_left = Self::dot(&self.weights[0..12], &state);
        let action_front = Self::dot(&self.weights[12..24], &state);
        let action_right = Self::dot(&self.weights[24..36], &state);

        let actions = [action_left, action_front, action_right];

        let max_action = actions
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        max_action
    }

    fn dot(v1: &[f32], v2: &[f32]) -> f32 {
        v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum()
    }
}
