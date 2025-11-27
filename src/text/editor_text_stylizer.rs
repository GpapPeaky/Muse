// Stylizer for general text in the editor

use macroquad::prelude::*;

use crate::text::editor_language_manager::*;
use crate::options::editor_pallete::*;

pub struct EditorGeneralTextStylizer {
    pub font: Font,
    pub font_size: u16,
    pub color: Color,
}

impl EditorGeneralTextStylizer {
    pub async fn new() -> EditorGeneralTextStylizer {
        EditorGeneralTextStylizer {
            font: load_ttf_font("assets/font/UbuntuMono-R.ttf").await.unwrap(),
            font_size: 18,
            color: WHITE,
        }
    }

    /// Calibrate the color of a token
    pub fn calibrate_string_color(
        &self,
        string: &str,
        elk: &EditorLanguageKeywords
    ) -> Color {
        if elk.control_flow.contains(&string) {
            return CONTROL_FLOW_COLOR;
        } else if elk.type_qualifiers.contains(&string) {
            return TYPE_QUALIFIER_COLOR;
        } else if elk.composite_types.contains(&string) {
            return COMPOSITE_TYPE_COLOR;
        } else if elk.storage_class.contains(&string) {
            return STORAGE_CLASS_COLOR;
        } else if elk.misc.contains(&string) {
            return MISC_COLOR;
        } else if elk.data_types.contains(&string) {
            return DATA_TYPE_COLOR;
        } else if string.chars().all(|c| c.is_ascii_digit()) {
            return NUMBER_LITERAL_COLOR;
        } else {
            return IDENTIFIER_COLOR;
        }
    }
    

    pub fn draw(
        &self,
        text: &str,
        x: f32,
        y: f32
    ){
        draw_text_ex(text, x, y,
            TextParams { font: Some(&self.font), font_size: self.font_size, color: self.color, ..Default::default() });
    }
}
