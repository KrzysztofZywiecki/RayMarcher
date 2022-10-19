mod vec3;
use vec3::*;
use image::{RgbImage};

#[derive(Debug)]
struct Camera {
    origin: Vec3,
    bottom_left: Vec3,
    right: Vec3,
    up: Vec3
}

#[derive(Debug)]
struct Ray {
    position: Vec3,
    direction: Vec3
}

impl Camera {
    fn new(origin: Vec3, yaw: f32, pitch: f32) -> Camera{
        let (p_sin, p_cos) = pitch.sin_cos();
        let (y_sin, y_cos) = yaw.sin_cos();

        let right = Vec3::new(y_cos, 0.0, y_sin);
        let up = Vec3::new(p_sin*y_cos, p_cos, -p_sin*y_cos);
        let forward = Vec3::new(-y_sin*p_cos, p_sin, p_cos*y_cos);

        Camera {
            origin,
            bottom_left: forward - up - right,
            right,
            up
        }
    }
}

fn cast_ray(camera: &Camera, x: f32, y: f32) -> Ray {
    Ray {
        position: camera.origin,
        direction: (camera.bottom_left + camera.up * y * 2.0 + camera.right * x * 2.0).normalize()
    }
    
}

struct Sphere {
    origin: Vec3,
    radious: f32
}

const EPSILON: f32 = 0.00001;

fn main() {

    let width = 1000;
    let height = 1000;

    let mut image = RgbImage::new(width, height);
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0);

    let s1 = Sphere{origin: Vec3::new(0.8, -0.3, 2.6), radious: 0.5};
    let s2 = Sphere{origin: Vec3::new(-0.5, 0.0, 3.0), radious: 1.0};
    let scene = vec![s1, s2];

    image.enumerate_pixels_mut().for_each(|(x, y, pixel)|{
        let x_coord = x as f32 / width as f32;
        let y_coord = y as f32 / height as f32;
        update_color(x_coord, y_coord, pixel, &camera, &scene)
    });

    image.save("image.png").unwrap();
}

fn update_color(x: f32, y: f32, pixel: &mut image::Rgb<u8>, camera: &Camera, scene: &[Sphere]) {
    let ray = cast_ray(camera, x, y);
    let color = march(0.001, 10.0, ray, scene, Vec3::new(0.4, -1.0, -0.4).normalize());

    *pixel = image::Rgb(color.u8_rgb());
}

fn distance_estimate(point: &Vec3, scene: &[Sphere]) -> f32 {
    let mut min_distance = f32::MAX;
    for sphere in scene.iter() {
        let dist = (*point - sphere.origin).len() - sphere.radious;

        if dist < min_distance {
            min_distance = dist
        }
    }
    min_distance
}

fn march(start: f32, stop: f32, ray: Ray, scene: &[Sphere], sun: Vec3) -> Vec3{
    let mut ray_origin = ray.position;
    let mut direction = ray.direction;

    let mut hit_geometry = false;
    let mut distance_travelled = start;
    while distance_travelled < stop {
        let distance = distance_estimate(&(ray_origin + direction * distance_travelled), scene);
        if distance < EPSILON {
            hit_geometry = true;
            break;
        }
        distance_travelled += distance;
    }
    if hit_geometry {
        let color = Vec3::new(1.0, 1.0, 1.0);
        let mut res: f32 = 1.0;
        ray_origin = ray_origin + direction * distance_travelled;
        direction = sun;
        distance_travelled = start;
        let mut previous_distance = EPSILON;
        while distance_travelled < stop {
            let distance = distance_estimate(&(ray_origin + direction * distance_travelled), scene);
            if distance < EPSILON {
                res = 0.0;
                break
            }
            let y = f32::powi(distance, 2) / (2.0 * previous_distance);
            let d = f32::sqrt(f32::powi(distance, 2) - f32::powi(y, 2));
            res = f32::min(res, 3.0*d/f32::max(0.0, distance-y));
            previous_distance = distance;
            distance_travelled += distance;
        }
        color*res
    }
    else {
        Vec3::new(0.0, 0.0, 0.0)
    }
}