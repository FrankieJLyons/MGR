use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub debug: bool,
    pub zoom: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self { debug: true, zoom: false }
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn toggle_zoom(&mut self) {
        self.zoom = !self.zoom;
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.toggle_debug();
        }
        if is_key_pressed(KeyCode::Z) {
            self.toggle_zoom();
        }
    }
}