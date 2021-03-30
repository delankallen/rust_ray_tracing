use rand::Rng;

use crate::{ray::Ray, utility_funcs::*, vec3::*};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u:Vec3,
    v:Vec3,
    lens_radius:f32
}

impl Camera {
    pub fn new(
        lookfrom:Point3,
        lookat:Point3,
        vup:Vec3,
        vfov:f32, 
        aspect_ratio:f32,
        aperture:f32,
        focus_dist:f32
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f32::tan(theta/2.0);        
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);
        
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist*w;
        let lens_radius = aperture / 2.0;

        Self { 
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u, v,
            lens_radius
        }
    }

    pub fn get_ray(&self, s:f32, t:f32, rng: &mut impl Rng) -> Ray {
        let rd:Vec3 = self.lens_radius * Vec3::random_in_unit_disc(rng);
        let offset = rd.x() * self.u + rd.y() * self.v;

        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
        }
    }
}