use crate::{
    material::Scatter,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Scatter,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hit for Vec<Box<dyn Hit>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut h: Option<HitRecord> = None;
        for hittable in self.iter() {
            if let Some(candidate) = hittable.hit(r, t_min, t_max) {
                match h {
                    None => h = Some(candidate),
                    Some(prev) => {
                        if candidate.t < prev.t {
                            h = Some(candidate);
                        }
                    }
                }
            }
        }
        h
    }
}
