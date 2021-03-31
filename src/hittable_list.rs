use rand::Rng;

use crate::{hittable::{Hittable, HitRecord}, ray::Ray};

// pub struct HittableList<T: ?Sized> {
//     pub objects: Vec<Box<dyn Hittable>>,
// }

pub trait HittableList: Send + Sync {
    fn hit_top<'a>(&'a self, ray: &Ray, rng: &mut impl Rng) -> Option<HitRecord<'a>>;
}

impl<'r, T: HittableList + ?Sized> HittableList for &'r T {
    fn hit_top<'a>(&'a self, ray: &Ray, rng: &mut impl Rng) -> Option<HitRecord<'a>> {
        (*self).hit_top(ray, rng)
    }
}

impl HittableList for Vec<Box<dyn Hittable>> {
    fn hit_top<'a>(&'a self, ray: &Ray, rng: &mut impl Rng) -> Option<HitRecord<'a>> {
        const NEAR: f32 = 0.001;
        let mut nearest = std::f32::INFINITY;
        let mut hit_record = None;

        for obj in self {
            if let Some(rec) = obj.hit(ray, NEAR..nearest, &mut || rng.gen()) {
                nearest = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}

// impl HittableList<dyn Hittable> {
//     pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
//         Self {
//             objects
//         }
//     }

//     // pub fn clear(&mut self) {
//     //     self.objects.clear();
//     // }

//     pub fn add(&mut self, item: Box<dyn Hittable>) {
//         self.objects.push(item);
//     }
// }

// impl Hittable for HittableList<dyn Hittable> {
//     fn hit(&self, r: &crate::ray::Ray, t_min:f32, t_max:f32) -> Option<HitRecord> {
//         let mut hit_record = None;
//         let mut closest_so_far = t_max;

//         for item in &self.objects {
//             if let Some(rec) = item.hit(r, t_min, closest_so_far) {
//                 closest_so_far = rec.t;
//                 hit_record = Some(rec);
//             }
//         }
        
//         hit_record
//     }
// }