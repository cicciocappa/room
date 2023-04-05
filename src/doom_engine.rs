use super::HEIGHT;
use super::WIDTH;

mod data_types;
mod map_renderer;
mod player;
mod wad_data;
mod wad_reader;
mod bsp;

use player::Player;
use wad_data::WadData;
use bsp::BSP;

pub struct KeyState {
    pub up:bool,
    pub down:bool,
    pub left:bool,
    pub right:bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

pub struct DoomEngine {
    wad_data: WadData,
    player: Player,
    pub key_state: KeyState,
}

impl DoomEngine {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        let wad_data = WadData::new("./doom1.wad", "E1M1\0\0\0\0");
        let player = Player {
            pos: wad_data.things[0].pos,
            angle: wad_data.things[0].angle,
        };
        let bsp = BSP {root_node_id: wad_data.nodes.len()-1};
        let key_state = KeyState{
            left:false,
            right:false,
            up:false,
            down:false,
            w: false,
            a: false,
            s: false,
            d: false,
        };
        DoomEngine { wad_data, player, key_state }
    }

     
    pub fn update(&mut self) {
        self.player.update(&self.key_state);
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    // min_x, min_y, max_x, max_y
    pub fn draw(&self, frame: &mut [u8]) {
        clear(frame);
        self.draw_lines(frame);
        self.draw_player(frame);
        self.draw_nodes(frame);
        
        
    }

    fn draw_lines(&self, frame: &mut[u8]){
        for l in self.wad_data.linedefs.iter() {
            let v1 = l.start_vertex_id as usize;
            let v2 = l.end_vertex_id as usize;
            let x0 = self.wad_data.vertexes[v1].x;
            let y0 = self.wad_data.vertexes[v1].y;
            let x1 = self.wad_data.vertexes[v2].x;
            let y1 = self.wad_data.vertexes[v2].y;
            for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
                plot(x, y, frame, (255,255,255));
            }
        }
    }
    fn draw_player(&self, frame: &mut [u8]){
        let cx = self.player.pos.0  ;
        let cy =  self.player.pos.1 ;
        for (x, y) in line_drawing::BresenhamCircle::new(cx, cy, 32) {
            plot(x,y,frame,(255,0,255));
        }
    }
    fn draw_nodes(&self, frame: &mut [u8]){
        //for n in self.wad_data.nodes.iter() {
            let n = &self.wad_data.nodes[self.wad_data.nodes.len()-1];
            let y0 = n.bbox_front.top;
            let x0 = n.bbox_front.left;
            let y1 = n.bbox_front.bottom;
            let x1 = n.bbox_front.right;
            for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y0)) {
                plot(x, y, frame, (255,0,255));
            }
            for (x, y) in line_drawing::Bresenham::new((x1, y0), (x1, y1)) {
                plot(x, y, frame, (255,0,255));
            }
            for (x, y) in line_drawing::Bresenham::new((x1, y1), (x0, y1)) {
                plot(x, y, frame, (255,0,255));
            }
            for (x, y) in line_drawing::Bresenham::new((x0, y1), (x0, y0)) {
                plot(x, y, frame, (255,0,255));
            } 
            let y0 = n.bbox_back.top;
            let x0 = n.bbox_back.left;
            let y1 = n.bbox_back.bottom;
            let x1 = n.bbox_back.right;
            for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y0)) {
                plot(x, y, frame, (128,128,0));
            }
            for (x, y) in line_drawing::Bresenham::new((x1, y0), (x1, y1)) {
                plot(x, y, frame, (128,128,0));
            }
            for (x, y) in line_drawing::Bresenham::new((x1, y1), (x0, y1)) {
                plot(x, y, frame, (128,128,0));
            }
            for (x, y) in line_drawing::Bresenham::new((x0, y1), (x0, y0)) {
                plot(x, y, frame, (128,128,0));
            } 
       //}
    }
}

fn plot(x: i16, y: i16, frame: &mut [u8], col:(u8,u8,u8)) {
    let sx = x as i32 / 8 + 160;
    let sy = -250 - y as i32 / 8;
    let p = (sx * 4 + sy * WIDTH as i32 * 4) as usize;
    if p > 0 && p < (WIDTH * HEIGHT * 4) as usize {
        frame[p] = col.0;
        frame[p + 1] = col.1;
        frame[p + 2] = col.2;
        frame[p + 3] = 255;
    }
}

 /// Clear the screen
fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}