use rand::prelude::*;

use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color},
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

pub struct Dielectric {
    pub ir: f64,
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let mut rng = thread_rng();

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction);

        let cos_theta = dot(&(-1.0 * unit_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflected = reflect(unit_direction, rec.normal);
        if (etai_over_etat * sin_theta) > 1.0 {
            return Some(ScatteredRay {
                attenuation,
                scattered: Ray::new(rec.p, reflected),
            });
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rng.gen::<f64>() < reflect_prob {
            return Some(ScatteredRay {
                attenuation,
                scattered: Ray::new(rec.p, reflected),
            });
        }
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        Some(ScatteredRay {
            attenuation,
            scattered: Ray::new(rec.p, refracted),
        })
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
