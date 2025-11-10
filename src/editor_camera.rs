use macroquad::window::{screen_height, screen_width};

use crate::editor_text::{FILE_TEXT_X_MARGIN, MODE_FONT_SIZE, MODE_Y_MARGIN};

pub struct EditorCamera {
    pub offset_x: f32,
    pub offset_y: f32,
    pub camera_w: f32,
    pub camera_h: f32,
}

impl EditorCamera {
    pub fn new() -> EditorCamera {
        EditorCamera {
            offset_x: 0.0,
            offset_y: 0.0,
            camera_w: screen_width(),
            camera_h: screen_height(),
        }
    }

     pub fn follow_cursor(&mut self, cursor_x_px: f32, cursor_y_px: f32) {
        let follow_margin = 50.0;

        let top_ui_height = MODE_Y_MARGIN + MODE_FONT_SIZE + 25.0;
        let left_ui_width = FILE_TEXT_X_MARGIN + 80.0;

        let min_x = self.offset_x + left_ui_width + follow_margin;
        let max_x = self.offset_x + self.camera_w - follow_margin;
        let min_y = self.offset_y + top_ui_height + follow_margin;
        let max_y = self.offset_y + self.camera_h - follow_margin;

        if cursor_x_px > max_x {
            self.offset_x = cursor_x_px - self.camera_w + follow_margin;
        } else if cursor_x_px < min_x {
            self.offset_x = (cursor_x_px - left_ui_width - follow_margin).max(0.0);
        }

        if cursor_y_px > max_y {
            self.offset_y = cursor_y_px - self.camera_h + follow_margin;
        } else if cursor_y_px < min_y {
            self.offset_y = (cursor_y_px - top_ui_height - follow_margin).max(0.0);
        }

        if self.offset_y < 1.0 {
            self.offset_y = 0.0;
        }
        if self.offset_x < 1.0 {
            self.offset_x = 0.0;
        }
    }

    pub fn world_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        (x - self.offset_x, y - self.offset_y)
    }
}
