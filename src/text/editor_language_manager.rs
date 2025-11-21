#[derive(Debug, Clone)]
pub struct EditorLanguageKeywords {
    pub control_flow: Vec<&'static str>,
    pub storage_class: Vec<&'static str>,
    pub type_qualifiers: Vec<&'static str>,
    pub composite_types: Vec<&'static str>,
    pub misc: Vec<&'static str>,
    pub data_types: Vec<&'static str>,
}

pub fn cpp_keywords() -> EditorLanguageKeywords {
    EditorLanguageKeywords {
        control_flow: vec![
            "if", "else", "switch", "case", "default",
            "for", "while", "do", "break", "continue",
            "goto", "return", "try", "catch", "finally",
        ],
        storage_class: vec![
            "auto", "static", "extern", "register", "typedef",
            "mutable", "constexpr", "thread_local",
        ],
        type_qualifiers: vec![
            "const", "volatile", "restrict", "constexpr",
        ],
        composite_types: vec![
            "struct", "union", "enum", "class",
        ],
        misc: vec![
            "sizeof", "inline", "virtual", "explicit",
            "namespace", "using", "operator", "template",
            "typename", "friend",
        ],
        data_types: vec![
            "int", "float", "double", "char", "void",
            "short", "long", "unsigned", "bool",
        ],
    }
}

pub fn java_keywords() -> EditorLanguageKeywords {
    EditorLanguageKeywords {
        control_flow: vec![
            "if", "else", "switch", "case", "default",
            "for", "while", "do", "break", "continue",
            "return", "try", "catch", "finally", "throw",
            "throws",
        ],
        storage_class: vec![
            "final", "abstract", "native", "static",
            "strictfp",
        ],
        type_qualifiers: vec![
            "volatile", "synchronized",
        ],
        composite_types: vec![
            "class", "interface", "enum", "record",
        ],
        misc: vec![
            "import", "package", "new", "instanceof",
            "extends", "implements",
        ],
        data_types: vec![
            "int", "float", "double", "boolean", "char",
            "short", "long", "byte",
        ],
    }
}

pub fn rust_keywords() -> EditorLanguageKeywords {
    EditorLanguageKeywords {
        control_flow: vec![
            "if", "else", "match", "loop", "while", "for",
            "break", "continue", "return",
        ],
        storage_class: vec![
            "static", "const", "mut",
        ],
        type_qualifiers: vec![
            "ref", "mut", "unsafe",
        ],
        composite_types: vec![
            "struct", "enum", "trait", "impl", "union",
        ],
        misc: vec![
            "crate", "super", "self", "pub", "use",
            "mod", "async", "await", "dyn",
        ],
        data_types: vec![
            "i8","i16","i32","i64","i128","isize",
            "u8","u16","u32","u64","u128","usize",
            "f32","f64","bool","char","str",
        ],
    }
}

pub fn load_keywords_for_extension(ext: &str) -> EditorLanguageKeywords {
    match ext {
        "c" | "h" | "cpp" | "hpp" | "cc" => cpp_keywords(),
        "java" => java_keywords(),
        "rs" => rust_keywords(),
        _ => EditorLanguageKeywords {
            control_flow: vec![],
            storage_class: vec![],
            type_qualifiers: vec![],
            composite_types: vec![],
            misc: vec![],
            data_types: vec![],
        }
    }
}

