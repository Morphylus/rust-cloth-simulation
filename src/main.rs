mod vec3;
mod simulation;
mod camera;

use macroquad::prelude::*;
use vec3::Vec3;
use simulation::Cloth;
use camera::Camera;

#[macroquad::main("Cloth Simulation 3D")]
async fn main() {
    let mass_value = 0.5;
    let dt = 0.02;
    let damping = 0.3;
    let wind = Vec3::new(-1.0, 0.0, -1.0);
    let wind_speed = 0.0;

    let mut cloth = Cloth::new(20, 20, 1.0, 100.0);
    let mut camera = Camera::new(vec3(30.0, 30.0, 30.0), vec3(0.0, 0.0, 0.0));

    loop {
        cloth.update(dt, damping, mass_value, wind, wind_speed);
        camera.update();
        clear_background(BLACK);
        camera.set_active();
        draw_scene(&cloth);
        set_default_camera();
        next_frame().await;
    }
}

fn draw_scene(cloth: &Cloth) {
    for spring in &cloth.structural_springs {
        let a = cloth.masses[spring.a].position;
        let b = cloth.masses[spring.b].position;
        draw_line_3d(a.into(), b.into(), WHITE);
    }

    for mass in &cloth.masses {
        draw_sphere(mass.position.into(), 0.1, None, RED);
    }

    let x_vec = Vec3::new(10.0, 0.0, 0.0);
    let y_vec = Vec3::new(0.0, 10.0, 0.0);
    let z_vec = Vec3::new(0.01, 0.0, 10.0);

    draw_line_3d(Vec3::zero().into(), x_vec.into(), RED);
    draw_line_3d(Vec3::zero().into(), y_vec.into(), GREEN);
    draw_line_3d(Vec3::zero().into(), z_vec.into(), BLUE);
}

impl Into<macroquad::prelude::Vec3> for Vec3 {
    fn into(self) -> macroquad::prelude::Vec3 {
        macroquad::prelude::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}