use crate::point::Point;
use crate::vector::Vector;

mod point;
mod vector;

fn equal(f1: f64, f2: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    if (f1 - f2).abs() < EPSILON {
        return true;
    }
    false
}

trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

#[test]
fn fire_virtual_cannon() {
    let velocity = Vector::new(1.0, 1.0, 0.0) * 100.0;
    // println!("Initial velocity: {:#?}", velocity);

    let mut projectile = Projectile::new(Point::new(0.0, 1.0, 0.0), velocity);

    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    println!();
    let mut i = 0;
    while projectile.position.y > 0.0 {
        tick(&environment, &mut projectile);
        i += 1;
        // eprintln!("{:#?}", projectile);
    }
    println!();
    println!("Cannonball went {:#?} meters!", projectile.position.x);
    println!("Finished after {} ticks", i);
}
