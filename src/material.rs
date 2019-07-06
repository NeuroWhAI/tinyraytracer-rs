use crate::vector::VecF;

pub struct Material {
    pub refractive_index: f32,
    pub albedo: VecF,
    pub diffuse: VecF,
    pub specular_exponent: f32,
}

impl Material {
    pub fn new(refractive_index: f32, albedo: VecF, diffuse: VecF, spec: f32) -> Self {
        Material {
            refractive_index,
            albedo,
            diffuse,
            specular_exponent: spec,
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Material {
            refractive_index: self.refractive_index,
            albedo: self.albedo.clone(),
            diffuse: self.diffuse.clone(),
            specular_exponent: self.specular_exponent,
        }
    }
}