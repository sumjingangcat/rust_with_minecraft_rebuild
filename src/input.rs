use glfw::{Action, Key, WindowEvent};
use nalgebra_glm::{vec2, DVec2};
use std::collections::HashMap;

pub struct InputCache {
    pub last_cursor_pos: DVec2,
    pub cursor_rel_pos: DVec2,
    pub key_states: HashMap<Key, Action>,
}

impl Default for InputCache {
    fn default() -> Self {
        InputCache {
            last_cursor_pos: vec2(0.0, 0.0),
            cursor_rel_pos: vec2(0.0, 0.0),
            key_states: HashMap::new(),
        }
    }
}

impl InputCache {
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            &glfw::WindowEvent::CursorPos(x, y) => {
                self.cursor_rel_pos.x = x - self.last_cursor_pos.x;
                self.cursor_rel_pos.y = y - self.last_cursor_pos.y;
                self.last_cursor_pos.x = x;
                self.last_cursor_pos.y = y;

                println!("Cursor pos: ({}, {})", x, y);
            }
            &glfw::WindowEvent::Key(key, _, action, _) => {
                self.key_states.insert(key, action);
            }
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        match self.key_states.get(&key) {
            Some(action) => *action == Action::Press || *action == Action::Repeat,
            None => false,
        }
    }
}
