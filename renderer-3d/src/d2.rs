type WindowBuffer<'a> = [u32];

pub struct Drawing<'a> {
    pub buffer: &'a mut WindowBuffer<'a>,
    pub width: u32
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

    pub fn middle_white_point(&mut self) {
        let height = self.buffer.len() as u32 / self.width;
    
        self.buffer[self.coord(self.width / 2, height / 2)] = rgb(255, 255, 255);
    }

    fn coord(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }
}

fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    (red as u32) << 16 | (green as u32) << 8 | blue as u32
}
