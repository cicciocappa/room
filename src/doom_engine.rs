 
use super::HEIGHT;
use super::WIDTH;

mod data_types;
mod map_renderer;
mod wad_data;
mod wad_reader;

use wad_data::WadData;

pub struct DoomEngine {
    wad_data: WadData,
}

impl DoomEngine {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        DoomEngine {
            wad_data: WadData::new("./doom1.wad", "E1M1\0\0\0\0"),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {}

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    // min_x, min_y, max_x, max_y
    pub fn draw(&self, frame: &mut [u8]) {
        for l in self.wad_data.linedefs.iter() {
            let v1 = l.start_vertex_id as usize;
            let v2 = l.end_vertex_id as usize;
            let x0 = self.wad_data.vertexes[v1].x;
            let y0 = self.wad_data.vertexes[v1].y;
            let x1 = self.wad_data.vertexes[v2].x;
            let y1 = self.wad_data.vertexes[v2].y;
            for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
                let sx = x as i32 / 8 + 140;
                let sy = -150 - y as i32 / 8 ;
                let p = (sx * 4 + sy * WIDTH as i32 * 4) as usize;
                if p > 0 && p < (WIDTH * HEIGHT * 4) as usize {
                    frame[p] = 255;
                    frame[p + 1] = 255;
                    frame[p + 2] = 255;
                    frame[p + 3] = 255;
                }
            }
            
        }
    }
}
