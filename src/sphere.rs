use crate::{
    hit::{Hit, HitRecord},
    material::Scatter,
    ray::Ray,
    vec3::{dot, Point3},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Box<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Box<dyn Scatter>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let mut hit_record = HitRecord {
            t,
            p,
            normal,
            mat: &*self.mat,
            front_face: false,
        };
        hit_record.set_face_normal(r, normal);
        return Some(hit_record);
    }
}
