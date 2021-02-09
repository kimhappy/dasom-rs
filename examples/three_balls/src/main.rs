use std::sync::Arc;
use image::Pixel;
use dasom::*;

type Rgb = image::Rgb< u8 >;
type ImageBuffer = image::ImageBuffer< Rgb, Vec< u8 > >;

fn color2rgb(c: Color) -> Rgb {
    let map = 255.0;
    image::Rgb([(c.r.sqrt() * map) as u8, (c.g.sqrt() * map) as u8, (c.b.sqrt() * map) as u8])
}

fn three_balls() -> shape::Group {
    let material_ground     = Arc::new(material::Lambertian::new(Color::from_24(0xF7F7E8)));
    let material_lambertian = Arc::new(material::Lambertian::new(Color::from_24(0xFFEEBB)));
    let material_dielectric = Arc::new(material::Dielectric::new(1.5));
    let material_metal      = Arc::new(material::Metal     ::new(Color::from_24(0xDDE0F0), 0.0));

    let ground      = Box::new(shape::Sphere::new(Vector::new( 0.0, -100.5, 0.0), 100.0 , material_ground    ));
    let lambertian  = Box::new(shape::Sphere::new(Vector::new(-1.0,    0.0, 0.0),   0.5 , material_lambertian));
    let dielectric1 = Box::new(shape::Sphere::new(Vector::new( 0.0,    0.0, 0.0),   0.5 , material_dielectric.clone()));
    let dielectric2 = Box::new(shape::Sphere::new(Vector::new( 0.0,    0.0, 0.0),  -0.45, material_dielectric));
    let metal       = Box::new(shape::Sphere::new(Vector::new( 1.0,    0.0, 0.0),   0.5 , material_metal     ));

    vec![ground, lambertian, dielectric1, dielectric2, metal]
}

fn main() {
    let ratio    = 16.0 / 9.0;
    let vfov     = std::f64::consts::PI / 9.0;
    let lrad     = 0.05;
    let lookfrom = Vector::new(0.0, 1.0, 5.0);
    let lookat   = Vector::new(0.0, 0.0, 0.0);
    let vup      = Vector::new(0.0, 1.0, 0.0);
    let fdist    = (lookfrom - lookat).norm();
    let ixsize   = 1280;
    let spp      = 512;
    let rdepth   = 64;
    let img_name = "three_balls";

    let start = std::time::Instant::now();

    let world  = three_balls();
    let camera = Camera::new(ratio, vfov, lrad, fdist, lookfrom, lookat, vup);
    let rt     = RayTracer::new(camera, ixsize, spp, rdepth);
    let buffer = rt.draw(&world).iter().map(|c| color2rgb(*c).channels().to_vec()).flatten().collect();
    let image  = ImageBuffer::from_vec(rt.ixsize, rt.iysize, buffer).unwrap();
    image.save(format!("{}({}x{}).png", img_name, rt.ixsize, rt.iysize)).unwrap();

    println!("Time elapsed: {:?}", start.elapsed());
}