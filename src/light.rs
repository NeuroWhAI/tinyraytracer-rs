use crate::vector::VecF;

pub struct Light {
    pub position: VecF,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: VecF, intensity: f32) -> Self {
        Light {
            position,
            intensity,
        }
    }
}