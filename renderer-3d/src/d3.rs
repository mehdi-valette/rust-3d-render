use crate::d2::{self, Vec2D};

#[derive(Clone)]
pub struct Object {
    vertices: Vec<Vec3D>,
    triangles: Vec<(usize, usize, usize)>
}

impl Object {

    pub fn new() -> Object {
        Object {
            vertices: vec![
                Vec3D::new(50, 50, 50),
                Vec3D::new(100, 50, 50),
                Vec3D::new(50, 100, 50),
                Vec3D::new(100, 100, 50),
                Vec3D::new(50, 50, 100),
                Vec3D::new(100, 50, 100),
                Vec3D::new(50, 100, 100),
                Vec3D::new(100, 100, 100)
            ],
            triangles: vec![
                (0, 1, 2),
                (1,2,3),

                (2,3,6),
                (3,6,7),

                (4,5,6),
                (5,6,7),


                (1,3,5),
                (3,5,7),

                (0,2,6),
                (0,4,6),
            ]
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let ref_vertice = &self.vertices.get(0).unwrap().clone();

        for vertice in self.vertices.iter_mut() {
            vertice.rotate_x(ref_vertice, angle);
        }
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let ref_vertice = &self.vertices.get(0).unwrap().clone();

        for vertice in self.vertices.iter_mut() {
            vertice.rotate_y(ref_vertice, angle);
        }
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let ref_vertice = &self.vertices.get(0).unwrap().clone();

        for vertice in self.vertices.iter_mut() {
            vertice.rotate_z(ref_vertice, angle);
        }
    }

    pub fn triangles(&self) -> &Vec<(usize, usize, usize)> {
        &self.triangles
    }

    pub fn vertices_in_triangle(&self, t: &(usize, usize, usize)) -> (&Vec3D, &Vec3D, &Vec3D) {
        (self.vertices.get(t.0).unwrap(), self.vertices.get(t.1).unwrap(), self.vertices.get(t.2).unwrap())
    }
}

#[derive(Clone)]
pub struct Vec3D {
    x: i32,
    y: i32,
    z: i32
}

impl Vec3D {
    pub fn new(x: i32, y: i32, z: i32) -> Vec3D {
        Vec3D{x, y, z}
    }

    pub fn to_2d(&self) -> Vec2D {
        d2::Vec2D::new(self.x, self.y)
    }

    fn substract(&mut self, reference: &Vec3D) {
        self.x -= reference.x;
        self.y -= reference.y;
        self.z -= reference.z;
    }

    fn add(&mut self, reference: &Vec3D) {
        self.x += reference.x;
        self.y += reference.y;
        self.z += reference.z;
    }

    pub fn rotate_x(&mut self, reference: &Vec3D, angle: f32) {
        self.substract(reference);
        let adapted_angle = -angle;

        let precise_y = self.y as f32;
        let precise_z = self.z as f32;

        self.y = (precise_y * adapted_angle.cos() - precise_z * adapted_angle.sin()) as i32;
        self.z = (precise_y * adapted_angle.sin() + precise_z * adapted_angle.cos()) as i32;

        self.add(reference);
    }


    pub fn rotate_y(&mut self, reference: &Vec3D, angle: f32) {
        self.substract(reference);
        let adapted_angle = -angle;

        let precise_z = self.z as f32;
        let precise_x = self.x as f32;

        self.z = (precise_z * adapted_angle.cos() - precise_x * adapted_angle.sin()) as i32;
        self.x = (precise_z * adapted_angle.sin() + precise_x * adapted_angle.cos()) as i32;

        self.add(reference);
    }

    pub fn rotate_z(&mut self, reference: &Vec3D, angle: f32) {
        self.substract(reference);
        let adapted_angle = -angle;

        let precise_x = self.x as f32;
        let precise_y = self.y as f32;

        self.x = (precise_x * adapted_angle.cos() - precise_y * adapted_angle.sin()) as i32;
        self.y = (precise_x * adapted_angle.sin() + precise_y * adapted_angle.cos()) as i32;

        self.add(reference);
    }
}