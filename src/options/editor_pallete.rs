use macroquad::prelude::*;

// Console
pub const CONSOLE_CONTAINER_COLOR: Color    = Color::from_hex(0x1B0B2A); // darker purple background
pub const CONSOLE_CURSOR_COLOR: Color       = Color::from_hex(0xFFFFFF); // keep white for visibility
pub const CONSOLE_TEXT_COLOR: Color         = Color::from_hex(0xFFB6F1); // brighter pink
pub const CONSOLE_FRAME_COLOR: Color        = Color::from_hex(0xFF00FF); // full magenta for contrast
pub const SELECTED_FILE_COLOR: Color        = Color::from_hex(0xFFD700); // brighter yellow-gold
pub const FOLDER_COLOR: Color               = Color::from_hex(0x00FFFF); // cyan, more vibrant
pub const FILE_COLOR: Color                 = Color::from_hex(0xFF6F00); // bright orange

// Text editor
pub const BACKGROUND_COLOR: Color           = Color::from_hex(0x200A30); // slightly richer purple
pub const COMPOSITE_TYPE_COLOR: Color       = Color::from_hex(0xFF00FF); // vivid magenta
pub const STORAGE_CLASS_COLOR: Color        = Color::from_hex(0xFF3399); // hot pink
pub const MISC_COLOR: Color                 = Color::from_hex(0xFFFFAA); // soft yellow, higher contrast
pub const TYPE_QUALIFIER_COLOR: Color       = Color::from_hex(0x00FFFF); // neon cyan
pub const CONTROL_FLOW_COLOR: Color         = Color::from_hex(0xFF3399); // same hot pink as storage class
pub const PUNCTUATION_COLOR: Color          = Color::from_hex(0xFFFF00); // bright yellow
pub const DATA_TYPE_COLOR: Color            = Color::from_hex(0xFF6F00); // bright orange
pub const NUMBER_LITERAL_COLOR: Color       = Color::from_hex(0x00CCFF); // more neon blue
pub const STRING_LITERAL_COLOR: Color       = Color::from_hex(0x00FFFF);
pub const CURSOR_COLOR: Color               = Color::from_hex(0xFFFFFF); // white
pub const MACRO_COLOR: Color                = Color::from_hex(0xFF66FF); // neon pink
pub const COMMENT_COLOR: Color              = Color::from_hex(0x00FF66); // bright lime green
pub const IDENTIFIER_COLOR: Color           = Color::from_hex(0xFF33CE); // vibrant pink-purple
