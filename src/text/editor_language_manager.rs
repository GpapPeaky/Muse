use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct EditorLanguageKeywords {
    pub control_flow: Vec<&'static str>,
    pub storage_class: Vec<&'static str>,
    pub type_qualifiers: Vec<&'static str>,
    pub composite_types: Vec<&'static str>,
    pub misc: Vec<&'static str>,
    pub data_types: Vec<&'static str>,
    pub _file_ids: Vec<String>,
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
        _file_ids: vec![],
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
        _file_ids: vec![],
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
            "ref", "mut", "unsafe",  "fn"
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
            "f32","f64","bool","char","str", "String"
        ],
        _file_ids: vec![],
    }
}

/// Load keywords for extensions based on the upper functions
pub fn load_keywords_for_extension(
    ext: &str
) -> EditorLanguageKeywords {
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
            _file_ids: vec![],
        }
    }
}

/// Check if a token is a keyword
/// return true if it is, false if not
pub fn _is_keyword(
    token: &str,
    elk: &EditorLanguageKeywords
) -> bool {
    if 
        elk.control_flow.contains(&token)
        || elk.storage_class.contains(&token)
        || elk.type_qualifiers.contains(&token)
        || elk.composite_types.contains(&token)
        || elk.misc.contains(&token)
        || elk.data_types.contains(&token) {
            return false;
    }
     
    true
}

/// Tokenize text file
/// from line-major to 
/// word-major, return the 
/// text file's tokens
pub fn _tokenize_text_file(
    text: &Vec<String>
) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut seen = HashSet::new();

    for line in text {
        for word in line.split_whitespace() {
            if seen.insert(word) {
                tokens.push(word.to_string());
            }
        }
    }

    tokens
}

/// Recognize identifiers
/// from the tokenized file text
/// pass the result into the
/// ELK's file identifier field. 
pub fn _recognize_identifiers(
    tokens: Vec<String>,
    elk: &mut EditorLanguageKeywords
) {
    for s in tokens {
        if _is_keyword(&s, elk) {
            elk._file_ids.push(s);
        }
    }
}
