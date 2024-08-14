use crate::vec3::Vec3;

const GRAVITY: Vec3 = Vec3{x: 0.0, y: -9.81, z:0.0};

pub struct Cloth {
    pub masses: Vec<Mass>,
    pub structural_springs: Vec<Spring>,
    shear_springs: Vec<Spring>,
    bend_springs: Vec<Spring>,
    rows: usize,
    cols: usize,
}

impl Cloth {
    pub fn new(cols: usize, rows: usize, spacing: f32, stiffness: f32) -> Self {
        let mut cloth = Cloth {
            masses: Vec::new(),
            structural_springs: Vec::new(),
            shear_springs: Vec::new(),
            bend_springs: Vec::new(),
            rows,
            cols,
        };
        cloth.init(spacing, stiffness);
        cloth
    }

    fn init(&mut self, spacing: f32, stiffness: f32) {
        // Init masses
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut new_point = Mass::new(
                    Vec3::new(j as f32 * spacing, 0.0, i as f32 * spacing),
                    false,
                );
                if i == 0 && j == 0 || i == 0 && j == self.cols - 1 {
                    new_point.pinned = true;
                }
                self.masses.push(new_point);
            }
        }

        // Init springs
        for i in 0..self.rows {
            for j in 0..self.cols {
                let index = i * self.cols + j;

                // Structural springs
                if j < self.cols - 1 {
                    self.structural_springs
                        .push(Spring::new(index, index + 1, spacing, stiffness));
                }

                if j > 0 {
                    self.structural_springs
                        .push(Spring::new(index, index - 1, spacing, stiffness));
                }

                if i > 0 {
                    self.structural_springs.push(Spring::new(
                        index,
                        index - self.cols,
                        spacing,
                        stiffness,
                    ));
                }

                if i < self.rows - 1 {
                    self.structural_springs.push(Spring::new(
                        index,
                        index + self.cols,
                        spacing,
                        stiffness,
                    ));
                }

                // Shear springs
                let shear_rest_length = (2.0 * spacing * spacing).sqrt();
                if j > 0 && i > 0 {
                    self.shear_springs.push(Spring::new(
                        index,
                        index - self.cols - 1,
                        shear_rest_length,
                        stiffness,
                    ));
                }

                if j < self.cols - 1 && i > 0 {
                    self.shear_springs.push(Spring::new(
                        index,
                        index - self.cols + 1,
                        shear_rest_length,
                        stiffness,
                    ));
                }

                if j > 0 && i < self.rows - 1 {
                    self.shear_springs.push(Spring::new(
                        index,
                        index + self.cols - 1,
                        shear_rest_length,
                        stiffness,
                    ));
                }

                if j < self.cols - 1 && i < self.cols - 1 {
                    self.shear_springs.push(Spring::new(
                        index,
                        index + self.cols + 1,
                        shear_rest_length,
                        stiffness,
                    ));
                }

                // Bend springs
                let bend_rest_length = spacing * 2.0;
                if i > 1 {
                    self.bend_springs.push(Spring::new(
                        index,
                        index - 2 * self.cols,
                        bend_rest_length,
                        stiffness,
                    ));
                }

                if j < self.cols - 2 {
                    self.bend_springs.push(Spring::new(
                        index,
                        index + 2,
                        bend_rest_length,
                        stiffness,
                    ));
                }

                if i < self.rows - 2 {
                    self.bend_springs.push(Spring::new(
                        index,
                        index + 2 * self.cols,
                        bend_rest_length,
                        stiffness,
                    ));
                }

                if j > 1 {
                    self.bend_springs.push(Spring::new(
                        index,
                        index - 2,
                        bend_rest_length,
                        stiffness,
                    ));
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32, damping: f32, mass_value: f32, wind: Vec3, wind_speed: f32) {
        self.apply_forces(damping);
        for mass in &mut self.masses {
            mass.update(dt, mass_value);
        }
    }

    fn apply_forces(&mut self, damping: f32) {
        for spring in &self.structural_springs {
            spring.apply_force(&mut self.masses);
        }

        for spring in &self.shear_springs {
            spring.apply_force(&mut self.masses);
        }

        for spring in &self.bend_springs {
            spring.apply_force(&mut self.masses);
        }

        for mass in &mut self.masses {
            mass.apply_force(GRAVITY); // gravity
            mass.apply_force(-damping * mass.velocity); // damping
        }
    }
}

pub struct Mass {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub pinned: bool,
}

impl Mass {
    pub fn new(position: Vec3, pinned: bool) -> Self {
        Mass {
            position: position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            pinned: pinned,
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        if !self.pinned {
            self.acceleration = self.acceleration + force;
        }
    }

    pub fn update(&mut self, dt: f32, mass: f32) {
        if !self.pinned {
            let new_acc = self.acceleration / mass;
            self.velocity = self.velocity + new_acc * dt;
            self.position = self.position + self.velocity * dt;
            self.acceleration = Vec3::zero();
        }
    }
}

pub struct Spring {
    pub a: usize,
    pub b: usize,
    pub rest_length: f32,
    pub stiffness: f32,
}

impl Spring {
    pub fn new(a: usize, b: usize, rest_length: f32, stiffness: f32) -> Self {
        Spring {
            a,
            b,
            rest_length,
            stiffness,
        }
    }

    pub fn apply_force(&self, masses: &mut [Mass]) {
        let distance = masses[self.b].position - masses[self.a].position;
        let length = distance.length();
        let force = self.stiffness * (length - self.rest_length) * distance.normalize();

        masses[self.a].apply_force(force);
        masses[self.b].apply_force(-force);
    }
}
