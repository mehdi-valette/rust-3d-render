type WindowBuffer<'a> = [u32];

pub struct Vec2D {
    x: i32,
    y: i32
}

impl Vec2D {
    pub fn new(x: i32, y: i32) -> Vec2D {
        Vec2D{x, y}
    }
    
    pub fn add(&mut self, point: &Vec2D) {
        self.x = self.x + point.x;
        self.y = self.y + point.y;
    }

    pub fn substract(&mut self, point: &Vec2D) {
        self.x = self.x - point.x;
        self.y = self.y - point.y;
    }

    pub fn to_index(&self, width: u32) -> Result<usize, &'static str> {
        if self.x < 0 || self.y < 0 {
            return Err("x and y must be >= 0");
        }

        Ok((self.y * width as i32 + self.x) as usize)
    }

    pub fn turn(&mut self, reference: &Vec2D, angle: f32) {
        self.substract(reference);
        let adapted_angle = -angle;

        let precise_x = self.x as f32;
        let precise_y = self.y as f32;

        self.x = (precise_x * adapted_angle.cos() - precise_y * adapted_angle.sin()) as i32;
        self.y = (precise_x * adapted_angle.sin() + precise_y * adapted_angle.cos()) as i32;

        self.add(reference);
    }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color{red, green, blue}
    }

    pub fn to_u32(&self) -> u32{
        (self.red as u32) << 16 | (self.green as u32) << 8 | self.blue as u32
    }
}

pub struct Drawing<'a> {
    buffer: &'a mut WindowBuffer<'a>,
    width: u32
}

impl<'a> Drawing<'a> {
    pub fn new(buffer: &'a mut WindowBuffer<'a>, width: u32) -> Drawing<'a> {
        Drawing{buffer, width}
    }

    pub fn all_black(&mut self) {
        for index in 0..self.buffer.len() {
            self.buffer[index as usize] = 0;
        }
    }

    pub fn draw_line(&mut self, point_1: &Vec2D, point_2: &Vec2D, color: &Color) {
        let (first_point, second_point) = {
            if point_1.x <= point_2.x {
                (point_1, point_2)
            } else {
                (point_2, point_1)
            }            
        };

        let delta_x = second_point.x - first_point.x;
        let delta_y = second_point.y as i32 - first_point.y as i32;

        if delta_x == 0 {
            let y_range = {
                if first_point.y < second_point.y {
                    first_point.y..=second_point.y
                } else {
                    second_point.y..=first_point.y
                }
            };

            for y in y_range {
                let coord = Vec2D::new(first_point.x, y);
                self.draw_pixel(&coord, &color)
            }

            return
        }

        let slope = delta_y as f32 / delta_x as f32;

        for x in 0..delta_x {
            let current_y = (x as f32 * slope) as i32;
            let next_y = ((x+1) as f32 * slope) as i32;

            // println!("delta x: {}, delta y: {}, slope: {}, x: {}, current y: {}, next y: {}", delta_x, delta_y, slope, x, current_y, next_y);

            let range_y = {
                if current_y < next_y {
                    current_y..=next_y
                } else {
                    next_y..=current_y
                }
            };

            for y in range_y {
                let mut coord = Vec2D::new(x, y);
                coord.add(&first_point);
                self.draw_pixel(&coord, &color)
            }
        }
    }

    pub fn draw_pixel(&mut self, coord: &Vec2D, color: &Color) {
        match coord.to_index(self.width) {
            Ok(index) => {
                self.buffer[index] = color.to_u32();
            },
            _ => {}
        }
    }
}
