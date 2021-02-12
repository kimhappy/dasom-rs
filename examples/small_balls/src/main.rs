use std::sync::Arc;
use image::Pixel;
use rand::Rng;
use dasom::*;

type Rgb = image::Rgb< u8 >;
type ImageBuffer = image::ImageBuffer< Rgb, Vec< u8 > >;

fn color2rgb(c: Color) -> Rgb {
    let map = 255.0;
    image::Rgb([(c.r.sqrt() * map) as u8, (c.g.sqrt() * map) as u8, (c.b.sqrt() * map) as u8])
}

fn random_pastel< R: Rng + ?Sized >(rng: &mut R) -> Color {
    Color::new(rng.gen_range(0.9..1.0), rng.gen_range(0.9..1.0), rng.gen_range(0.9..1.0))
}

fn small_balls() -> shape::Group {
    let mut world = (-30..10).flat_map(|x|
        (-20..20).map(move |z| -> Box< dyn Shape + Send + Sync > {
            let mut rng = rand::thread_rng();
            let mat_num = rng.gen();
            let cen = Vector::new(x as Real + 0.8 * rand::random::< Real >(), 0.2, z as Real + 0.8 * rand::random::< Real >());
            let mat: Arc< dyn Material + Send + Sync > =
                     if (0.0..0.7).contains(&mat_num) { Arc::new(material::Lambertian::new(random_pastel(&mut rng))) }
                else if (0.7..0.9).contains(&mat_num) { Arc::new(material::Metal     ::new(random_pastel(&mut rng), rng.gen_range(0.0..0.01))) }
                else                                       { Arc::new(material::Dielectric::new(1.5)) };
            Box::new(shape::Sphere::new(cen, 0.2, mat)) })).collect::< Vec< _ > >();

    let material_ground     = Arc::new(material::Lambertian::new(Color::from_24(0xF7F7E8)));
    let material_lambertian = Arc::new(material::Lambertian::new(Color::from_24(0xFFEEBB)));
    let material_dielectric = Arc::new(material::Dielectric::new(1.5));
    let material_metal      = Arc::new(material::Metal     ::new(Color::from_24(0xDDE0F0), 0.0));

    let ground     = Box::new(shape::Sphere::new(Vector::new( 0.0, -1000.0, 0.0), 1000.0, material_ground    ));
    let lambertian = Box::new(shape::Sphere::new(Vector::new(-4.0,     1.0, 0.0),    1.0, material_lambertian));
    let dielectric = Box::new(shape::Sphere::new(Vector::new( 0.0,     1.0, 0.0),    1.0, material_dielectric));
    let metal      = Box::new(shape::Sphere::new(Vector::new( 4.0,     1.0, 0.0),    1.0, material_metal     ));

    world.push(ground);
    world.push(lambertian);
    world.push(dielectric);
    world.push(metal);
    world
}

fn main() {
    let ratio    = 16.0 / 9.0;
    let vfov     = std::f64::consts::PI / 9.0;
    let lrad     = 0.05;
    let lookfrom = Vector::new(14.0, 3.0, 3.0);
    let lookat   = Vector::new(-2.5, 0.0, 0.0);
    let vup      = Vector::new( 0.0, 1.0, 0.0);
    let fdist    = 10.0;
    let ixsize   = 1280;
    let spp      = 512;
    let rdepth   = 64;
    let img_name = "small_balls";

    let start = std::time::Instant::now();

    let world  = small_balls();
    let camera = Camera::new(ratio, vfov, lrad, fdist, lookfrom, lookat, vup);
    let rt     = RayTracer::new(camera, ixsize, spp, rdepth);
    let buffer = rt.draw(&world).iter().map(|c| color2rgb(*c).channels().to_vec()).flatten().collect();
    let image  = ImageBuffer::from_vec(rt.ixsize, rt.iysize, buffer).unwrap();
    image.save(format!("{}({}x{}).png", img_name, rt.ixsize, rt.iysize)).unwrap();

    println!("Time elapsed: {:?}", start.elapsed());
}