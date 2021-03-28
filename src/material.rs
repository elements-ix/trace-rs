use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Color},
};

pub struct ScatteredRay {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let scatter_direction = rec.normal + random_unit_vector();
        Some(ScatteredRay {
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(unit_vector(r_in.direction), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        if dot(&scattered.direction, &rec.normal) > 0.0 {
            return Some(ScatteredRay {
                scattered,
                attenuation: self.albedo,
            });
        }

        None
    }
}
