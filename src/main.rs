mod vector;
mod sphere;
mod material;
mod light;

use std::{
    f32,
    fs::File,
    io::prelude::*,
};
use crate::{
    vector::{VecF, reflect},
    sphere::Sphere,
    material::Material,
    light::Light,
};

fn refract(i: &VecF, n: &VecF, refractive_index: f32) -> VecF {
    let mut cosi = -i.dot(n).min(1.0).max(-1.0);
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let mut nn = n.clone();

    if cosi < 0.0 {
        cosi = -cosi;
        std::mem::swap(&mut etai, &mut etat);
        nn = n * -1.0;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        VecF::from_slice(&[0.0, 0.0, 0.0])
    }
    else {
        &(i * eta) + &(&nn * (eta * cosi - k.sqrt()))
    }
}

fn scene_intersect(orig: &VecF, dir: &VecF, sphere_list: &Vec<Sphere>)
    -> (bool, VecF, VecF, Material) {

    let mut dist = f32::MAX;
    let mut hit = VecF::new(3);
    let mut n = VecF::new(3);
    let mut material = Material::new(
        1.0,
        VecF::from_slice(&[1.0, 0.0, 0.0, 0.0]),
        VecF::new(3),
        0.0);

    for sphere in sphere_list {
        let (intersect, dist_i) = sphere.ray_intersect(orig, dir);

        if intersect && dist_i < dist {
            dist = dist_i;
            hit = orig + &(dir * dist_i);
            n = (&hit - &sphere.center).normalize();
            material = sphere.material.clone();
        }
    }

    let mut checkerboard_dist = f32::MAX;
    if dir[1].abs() > 1e-3 {
        let d = -(orig[1] + 4.0) / dir[1];
        let pt = orig + &(dir * d);
        if d > 0.0 && pt[0].abs() < 10.0 && pt[2] < -10.0 && pt[2] > -30.0 && d < dist {
            let diffuse_flag = ((pt[0] * 0.5 + 1000.0) as i32 + (pt[2] * 0.5) as i32) & 1;

            checkerboard_dist = d;
            hit = pt;
            n = VecF::from_slice(&[0.0, 1.0, 0.0]);
            material.diffuse = if diffuse_flag == 0 {
                &VecF::from_slice(&[1.0, 0.7, 0.3]) * 0.3
            }
            else {
                &VecF::from_slice(&[1.0, 1.0, 1.0]) * 0.3
            };
        }
    }

    (dist.min(checkerboard_dist) < 1000.0, hit, n, material)
}

fn cast_ray(orig: &VecF, dir: &VecF, sphere_list: &Vec<Sphere>, light_list: &Vec<Light>, depth: usize)
    -> VecF {

    let (intersect, point, n, material) = scene_intersect(orig, dir, sphere_list);

    if depth <= 4 && intersect {
        let reflect_dir = reflect(dir, &n).normalize();
        let refract_dir = refract(dir, &n, material.refractive_index).normalize();
        let reflect_orig = if reflect_dir.dot(&n) < 0.0 {
            &point - &(&n * 1e-3)
        }
        else {
            &point + &(&n * 1e-3)
        };
        let refract_orig = if refract_dir.dot(&n) < 0.0 {
            &point - &(&n * 1e-3)
        }
        else {
            &point + &(&n * 1e-3)
        };
        let reflect_color = cast_ray(&reflect_orig, &reflect_dir, sphere_list, light_list, depth + 1);
        let refract_color = cast_ray(&refract_orig, &refract_dir, sphere_list, light_list, depth + 1);

        let mut diffuse_light: f32 = 0.0;
        let mut spec_light: f32 = 0.0;

        for light in light_list {
            let light_dir = (&light.position - &point).normalize();
            let light_distance = (&light.position - &point).norm();

            let shadow_orig = if light_dir.dot(&n) < 0.0 {
                &point - &(&n * 1e-3)
            }
            else {
                &point + &(&n * 1e-3)
            };
            let shadow_test = scene_intersect(&shadow_orig, &light_dir, sphere_list);
            let (shadow_intersect, shadow_pt, _shadow_n, _tmp_material) = shadow_test;

            if shadow_intersect && (&shadow_pt - &shadow_orig).norm() < light_distance {
                continue;
            }

            diffuse_light += light.intensity * n.dot(&light_dir).max(0.0);
            spec_light += (&reflect(&(&light_dir * -1.0), &n) * -1.0).dot(dir).max(0.0)
                .powf(material.specular_exponent) * light.intensity;
        }

        let spec_val = spec_light * material.albedo[1];

        &(&(&(&material.diffuse * (diffuse_light * material.albedo[0]))
            + &VecF::from_slice(&[spec_val, spec_val, spec_val]))
            + &(&reflect_color * material.albedo[2]))
            + &(&refract_color * material.albedo[3])
    }
    else {
        VecF::from_slice(&[0.2, 0.7, 0.8])
    }
}

fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>) {
    let width = 1024;
    let height = 768;
    let fov = f32::consts::PI / 3.0;
    let orig = VecF::from_slice(&[0.0, 0.0, 0.0]);
    let mut framebuffer = Vec::with_capacity(width * height);

    let f_width = width as f32;
    let f_height = height as f32;
    let tan_fov = (fov / 2.0).tan();

    // Render pixels.
    for y in 0..height {
        let vy = -(2.0 * (y as f32 + 0.5) / f_height - 1.0) * tan_fov;

        for x in 0..width {
            let vx = (2.0 * (x as f32 + 0.5) / f_width - 1.0) * tan_fov * f_width / f_height;

            let dir = VecF::from_slice(&[vx, vy, -1.0]).normalize();

            framebuffer.push(cast_ray(&orig, &dir, spheres, lights, 0));
        }
    }

    // Save the framebuffer to file.
    if let Ok(mut img) = File::create("out.ppm") {
        img.write_fmt(format_args!("P6\n{} {}\n255\n", width, height))
            .expect("Can't write out.ppm");

        for p in 0..framebuffer.len() {
            let c = &mut framebuffer[p];
            let max = c[0].max(c[1].max(c[2]));

            if max > 1.0 {
                *c = &*c * (1.0 / max);
            }

            for axis in 0..3 {
                let color = framebuffer[p][axis].min(1.0).max(0.0);
                img.write(&[(color * 255.0) as u8])
                    .expect("Can't write out.ppm");
            }
        }
    }
    else {
        println!("Can't create out.ppm");
    }
}

fn main() {
    let ivory = Material::new(
        1.0,
        VecF::from_slice(&[0.6, 0.3, 0.1, 0.0]),
        VecF::from_slice(&[0.4, 0.4, 0.3]),
        50.0);
    let glass = Material::new(
        1.5,
        VecF::from_slice(&[0.0, 0.5, 0.1, 0.8]),
        VecF::from_slice(&[0.6, 0.7, 0.8]),
        125.0);
    let red_rubber = Material::new(
        1.0,
        VecF::from_slice(&[0.9, 0.1, 0.0, 0.0]),
        VecF::from_slice(&[0.3, 0.1, 0.1]),
        10.0);
    let mirror = Material::new(
        1.0,
        VecF::from_slice(&[0.0, 10.0, 0.8, 0.0]),
        VecF::from_slice(&[1.0, 1.0, 1.0]),
        1425.0);

    let spheres = vec![
        Sphere::new(VecF::from_slice(&[-3.0, 0.0, -16.0]), 2.0, ivory.clone()),
        Sphere::new(VecF::from_slice(&[-1.0, -1.5, -12.0]), 2.0, glass.clone()),
        Sphere::new(VecF::from_slice(&[1.5, -0.5, -18.0]), 3.0, red_rubber.clone()),
        Sphere::new(VecF::from_slice(&[7.0, 5.0, -18.0]), 4.0, mirror.clone()),
    ];

    let lights = vec![
        Light::new(VecF::from_slice(&[-20.0, 20.0, 20.0]), 1.5),
        Light::new(VecF::from_slice(&[30.0, 50.0, -25.0]), 1.8),
        Light::new(VecF::from_slice(&[30.0, 20.0, 30.0]), 1.7),
    ];

    render(&spheres, &lights);
}
