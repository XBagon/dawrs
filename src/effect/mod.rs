pub trait Effect {
    fn process(&mut self, samples: &mut Vec<f32>);
}
