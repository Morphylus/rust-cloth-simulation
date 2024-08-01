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
        Mass { position: position, velocity: Vec3::zero(), acceleration: Vec3::zero(), pinned: pinned}
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
            self.acceleration = Vec3::zero();
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
    let rows = 15;
    let cols = 15;
    let spacing = 2.0;
    let stiffness = 100.0;
    let rest_length = spacing;
    let mass_value = 0.5;
    let dt = 0.01;
    let damping = 0.3;
    let wind = Vec3::new(-1.0, 0.0, -1.0);
    let wind_speed = 5.0;
    
    let mut masses: Vec<Mass> = Vec::new();
    let mut structural_springs: Vec<Spring> = Vec::new();
    let mut shear_springs: Vec<Spring> = Vec::new();
    let mut bend_springs: Vec<Spring> = Vec::new();

    // Init masses
    for i in 0..rows {
        for j in 0..cols {
            let mut new_point = Mass::new(Vec3::new(j as f64 * spacing, 0.0, i as f64 * spacing), false);
            if i == 0 && j == 0 {
                new_point.pinned = true;
            }
            masses.push(new_point);
        }
    }

    // Init springs
    for i in 0..rows {
        for j in 0..cols {
            let index = i * cols + j;

            // Structural springs
            if j < cols - 1 {
                structural_springs.push(Spring::new(index, index + 1, rest_length, stiffness));
            }

            if j > 0 {
                structural_springs.push(Spring::new(index, index - 1, rest_length, stiffness));
            }

            if i > 0 {
                structural_springs.push(Spring::new(index, index - cols, rest_length, stiffness));
            }

            if i < rows - 1 {
                structural_springs.push(Spring::new(index, index + cols, rest_length, stiffness));
            }

            // Shear springs
            let shear_rest_length = (2.0 * rest_length * rest_length).sqrt();
            if j > 0 && i > 0 {
                shear_springs.push(Spring::new(index, index - cols - 1, shear_rest_length, stiffness));
            }

            if j < cols - 1 && i > 0 {
                shear_springs.push(Spring::new(index, index - cols + 1, shear_rest_length, stiffness));
            }

            if j > 0 && i < rows - 1 {
                shear_springs.push(Spring::new(index, index + cols - 1, shear_rest_length, stiffness));
            }

            if j < cols - 1 && i < cols - 1 {
                shear_springs.push(Spring::new(index, index + cols + 1, shear_rest_length, stiffness));
            }

            // Bend springs
            let bend_rest_length = rest_length * 2.0;
            if i > 1 {
                bend_springs.push(Spring::new(index, index - 2 * cols, bend_rest_length, stiffness));
            }

            if j < cols - 2 {
                bend_springs.push(Spring::new(index, index + 2, bend_rest_length, stiffness));
            }

            if i < rows - 2 {
                bend_springs.push(Spring::new(index, index + 2 * cols, bend_rest_length, stiffness));
            }

            if j > 1 {
                bend_springs.push(Spring::new(index, index - 2, bend_rest_length, stiffness));
            }


        }
    }

    loop {
        // Update simulation
        for spring in &structural_springs {
            spring.apply_force(&mut masses);
        }

        for spring in &shear_springs {
            spring.apply_force(&mut masses);
        }
        
        let positions: Vec<Vec3> = masses.iter().map(|mass| mass.position).collect();

        for (index, mass) in masses.iter_mut().enumerate() {
            mass.apply_force(Vec3::new(0.0, -9.81, 0.0)); // gravity
            mass.apply_force(-damping * mass.velocity); // Damping

            let mut surrounding_positions: Vec<Vec3> = Vec::new();
            let i = index / cols;
            let j = index % cols;

            // Structural springs

            if i < rows - 1 {
                surrounding_positions.push(positions[index + cols]);
            }

            if j < cols - 1 {
                surrounding_positions.push(positions[index + 1]);
            }

            if i > 0 {
                surrounding_positions.push(positions[index - cols]);
            }

            if j > 0 {
                surrounding_positions.push(positions[index - 1]);
            }

            let vertex_normal = calculate_vertex_normal(&mass.position, &surrounding_positions);

            mass.apply_force(wind_speed * vertex_normal.dot(wind - mass.velocity) * vertex_normal);

            mass.update(dt, mass_value);
        }

        // Draw
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(50.0, 50.0, 50.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        });

        for spring in &structural_springs {
            let a = masses[spring.a].position;
            let b = masses[spring.b].position;
            draw_line_3d(a.into(), b.into(), WHITE);
        }

        for mass in &masses {
            draw_sphere(mass.position.into(), 0.1, None, RED);
        }

        let x_vec = Vec3::new(10.0, 0.0, 0.0);
        let y_vec = Vec3::new(0.0, 10.0, 0.0);
        let z_vec = Vec3::new(0.01, 0.0, 10.0);

        draw_line_3d(Vec3::zero().into(), x_vec.into(), RED);
        draw_line_3d(Vec3::zero().into(), y_vec.into(), GREEN);
        draw_line_3d(Vec3::zero().into(), z_vec.into(), BLUE);

        set_default_camera();

        next_frame().await;
    }
}

impl Into<macroquad::prelude::Vec3> for Vec3 {
    fn into(self) -> macroquad::prelude::Vec3 {
        macroquad::prelude::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

fn calculate_vertex_normal(center: &Vec3, surrounding_positions: &Vec<Vec3>) -> Vec3 {
    let mut triangle_normals = Vec::new();
    let length = surrounding_positions.len();
    for i in 0..length {
        let neighbors = (surrounding_positions[i], surrounding_positions[(i + 1) % length]);
        triangle_normals.push(calculate_triangle_normal(*center, neighbors.0, neighbors.1));
    }

    let mut res = Vec3::zero();
    for normal in triangle_normals {
        res = res + normal;
    }
    res / length as f64
}

fn calculate_triangle_normal(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec3 {
    let u = p2 - p1;
    let v = p3 - p1;

    Vec3::new(u.y * v.z - u.z * v.y, u.z * v.x - u.x * v.z, u.x * v.y - u.y * v.x)
}
