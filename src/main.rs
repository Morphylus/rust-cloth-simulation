mod vec3;

use macroquad::prelude::*;
use vec3::Vec3;

struct Mass {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    pinned: bool
}

impl Mass {
    fn new(position: Vec3, pinned: bool) -> Self {
        Mass { position: position, velocity: Vec3::ZERO(), acceleration: Vec3::ZERO(), pinned: pinned}
    }

    fn apply_force(&mut self, force: Vec3) {
        if !self.pinned {
            self.acceleration = self.acceleration + force;
        }
    }

    fn update(&mut self, dt: f64, mass: f64) {
        if !self.pinned {
            let new_acc = self.acceleration / mass;
            self.velocity = self.velocity + new_acc * dt;
            self.position = self.position + self.velocity * dt;
            self.acceleration = Vec3::ZERO();
        }
    }
}

struct Spring {
    a: usize,
    b: usize,
    rest_length: f64,
    stiffness: f64
}

impl Spring {
    fn new(a: usize, b: usize, rest_length: f64, stiffness: f64) -> Self {
        Spring {
            a,
            b,
            rest_length,
            stiffness
        }
    }

    fn apply_force(&self, masses: &mut [Mass]) {
        let distance = masses[self.b].position - masses[self.a].position;
        let length = distance.length();
        let force = self.stiffness * (length - self.rest_length) * distance.normalize();

        masses[self.a].apply_force(force);
        masses[self.b].apply_force(-force);
    }
}

#[macroquad::main("Cloth Simulation 3D")]
async fn main() {
    let rows = 10;
    let cols = 10;
    let spacing = 2.0;
    let stiffness = 300.0;
    let rest_length = spacing;
    let mass_value = 1.0;
    let dt = 0.03;
    let damping = 0.3;
    
    let mut masses: Vec<Mass> = Vec::new();
    let mut springs: Vec<Spring> = Vec::new();

    // Init masses
    for i in 0..rows {
        for j in 0..cols {
            let mut new_point = Mass::new(Vec3::new(j as f64 * spacing, 0.0, i as f64 * spacing), false);
            if (i == 0 && j == 0) || (i == rows-1 && j == 0) {
                new_point.pinned = true;
            }
            masses.push(new_point);
        }
    }

    // Init springs
    for i in 0..rows {
        for j in 0..cols {
            let index = i * cols + j;

            if j < cols - 1 {
                springs.push(Spring::new(index, index + 1, rest_length, stiffness));
            }

            if i < rows - 1 {
                springs.push(Spring::new(index, index + cols, rest_length, stiffness));
            }
        }
    }

    loop {
        // Update simulation
        for spring in &springs {
            spring.apply_force(&mut masses);
        }

        for mass in &mut masses {
            mass.apply_force(Vec3::new(0.0, -9.81, 0.0)); // gravity
            mass.apply_force(-damping * mass.velocity); // Damping
            mass.update(dt, mass_value);
        }

        // Draw
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(60.0, -50.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        });

        for spring in &springs {
            let a = masses[spring.a].position;
            let b = masses[spring.b].position;
            draw_line_3d(a.into(), b.into(), WHITE);
        }

        for mass in &masses {
            draw_sphere(mass.position.into(), 0.1, None, RED);
        }

        set_default_camera();

        next_frame().await;
    }
}

impl Into<macroquad::prelude::Vec3> for Vec3 {
    fn into(self) -> macroquad::prelude::Vec3 {
        macroquad::prelude::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}
