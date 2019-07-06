use crate::vector::VecF;
use crate::material::Material;

pub struct Sphere {
    pub center: VecF,
    radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: VecF, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Sphere {
    pub fn ray_intersect(&self, orig: &VecF, dir: &VecF) -> (bool, f32) {
        let l = &self.center - orig;
        let tca = l.dot(dir);
        let d2 = l.dot(&l) - tca * tca;
        if d2 > self.radius * self.radius {
            return (false, 0.0);
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        (!(t0 < 0.0), t0)
    }
}