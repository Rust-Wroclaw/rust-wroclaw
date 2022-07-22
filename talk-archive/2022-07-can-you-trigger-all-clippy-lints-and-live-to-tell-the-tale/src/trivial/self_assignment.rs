pub struct V2i {
    x: i32,
    y: i32,
}

impl V2i {
    pub fn replace(&mut self, other: &V2i) {
        self.x = other.x;
        self.y = self.y;
    }
}
