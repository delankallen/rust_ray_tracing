use crate::{hittable::{Hittable, HitRecord}};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self {
            objects
        }
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.objects.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for item in &self.objects {
            if let Some(rec) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        
        hit_record
    }
}