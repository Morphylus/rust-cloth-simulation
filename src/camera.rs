use macroquad::prelude::*;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3) -> Self {
        Camera {
            position,
            target,
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            aspect: screen_width() / screen_height(),
        }
    }

    pub fn update(&mut self) {
        // Update camera position based on user input
        let move_speed = 1.0;
        if is_key_down(KeyCode::W) {
            self.position += (self.target - self.position).normalize() * move_speed;
        }
        if is_key_down(KeyCode::S) {
            self.position -= (self.target - self.position).normalize() * move_speed;
        }
        if is_key_down(KeyCode::A) {
            let right = (self.target - self.position).cross(self.up).normalize();
            self.position -= right * move_speed;
        }
        if is_key_down(KeyCode::D) {
            let right = (self.target - self.position).cross(self.up).normalize();
            self.position += right * move_speed;
        }

        // Update aspect ratio in case window was resized
        self.aspect = screen_width() / screen_height();
    }

    pub fn set_active(&self) {
        set_camera(&Camera3D {
            position: self.position,
            target: self.target,
            up: self.up,
            fovy: self.fovy,
            aspect: Some(self.aspect),
            ..Default::default()
        });
    }
}