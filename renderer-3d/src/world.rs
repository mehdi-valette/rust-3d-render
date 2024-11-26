use std::f32::consts::PI;

use crate::{d2, d3};

pub struct World {
    object: d3::Object,
    object_rotated: d3::Object,
    color: d2::Color,
    step: u32
}

impl World {
    pub fn new(color: d2::Color) -> World {
        let object = d3::Object::new();

        World {color, object: object.clone(), object_rotated: object.clone(), step: 0}
    }

    pub fn next_step(&mut self) {
        self.step += 1;

        self.object_rotated = self.object.clone();

        self.object_rotated.rotate_x((PI / 100.0) * self.step as f32);
        self.object_rotated.rotate_y((PI / 100.0) * self.step as f32);
        self.object_rotated.rotate_z((PI / 100.0) * self.step as f32);
    }

    pub fn draw(&mut self, buffer: &mut [u32], width: u32) {
        let mut drawing = d2::Drawing::new(buffer.as_mut(), width);

        for triangle in self.object_rotated.triangles() {
            let (point_1, point_2, point_3) = self.object_rotated.vertices_in_triangle(triangle);
            drawing.draw_line(&point_1.to_2d(), &point_2.to_2d(), &self.color);
            drawing.draw_line(&point_2.to_2d(), &point_3.to_2d(), &self.color);
            drawing.draw_line(&point_3.to_2d(), &point_1.to_2d(), &self.color);
        }
    }
}