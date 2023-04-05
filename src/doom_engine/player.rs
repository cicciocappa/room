use super::KeyState;

pub struct Player {
    pub pos: (i16,i16),
    pub angle: i16,
}

impl Player {
    pub fn update(&mut self, key_state: &KeyState){
        self.control(key_state);
    }

    fn control(&mut self, key_state: &KeyState){
        if key_state.w {
            self.pos.1 += 2;
        }
        if key_state.s {
            self.pos.1 -= 2;
        }
        if key_state.a {
            self.pos.0 -= 2;
        }
        if key_state.d {
            self.pos.0 += 2;
        }
        if key_state.left {
            self.angle += 2;
        }
        if key_state.right {
            self.angle -= 2;
        }
    }
}