
pub const CONTROL_FLOW_STATEMENTS: [&str; 46] = [
    "if", "else", "switch", "case", "default",
    "for", "while", "do", "break", "continue",
    "goto", "return", "try", "catch", "finally",
    "throw", "throws", "loop", "match", "yield",
    "await", "async", "then", "except", "raise",
    "elif", "when", "until", "unless", "foreach",
    "in", "from", "select", "where", "defer",
    "guard", "assert", "panic", "recover",
    "next", "redo", "exit", "abort", "with",
    "elif", "end",
];

pub const STORAGE_CLASS_SPECIFIERS: [&str; 18] = [
    "auto", "static", "extern", "register", "typedef",
    "mutable", "constexpr", "thread_local", "let", "var",
    "const", "final", "override", "sealed", "lazy",
    "owned", "borrowed", "inline",
];

pub const TYPE_QUALIFIERS: [&str; 14] = [
    "const", "volatile", "restrict", "constexpr",
    "ref", "mut", "transient", "synchronized",
    "abstract", "readonly", "immutable", "dynamic",
    "weak", "unsafe",
];

pub const COMPOSITE_TYPES: [&str; 12] = [
    "struct", "union", "enum", "class", "trait",
    "interface", "protocol", "record", "object",
    "impl", "concept", "module",
];

pub const MISC: [&str; 39] = [
    "sizeof", "inline", "virtual", "explicit", "namespace",
    "using", "operator", "template", "typename", "friend",
    "crate", "super", "self", "import", "package",
    "include", "public", "private", "protected", "internal",
    "static_cast", "reinterpret_cast", "dynamic_cast", "const_cast",
    "typeof", "instanceof", "new", "delete", "clone",
    "as", "is", "extends", "implements", "default",
    "partial", "module", "export", "require", "use",
];

pub const DATA_TYPES: [&str; 60] = [
    "int", "float", "double", "char", "void",
    "short", "long", "signed", "unsigned", "bool",
    "boolean", "byte", "wchar_t", "auto", "decltype",
    "nullptr_t", "String", "str", "u8", "u16",
    "u32", "u64", "u128", "i8", "i16", "i32",
    "i64", "i128", "f32", "f64", "usize", "isize",
    "any", "object", "None", "null", "undefined",
    "map", "list", "array", "tuple", "set", "dict",
    "Vec", "Option", "Result", "number", "char8_t",
    "char16_t", "char32_t", "interface", "record", "trait",
    "enum", "struct", "unit", "string", "symbol",
    "function", "object",
];

#[allow(dead_code)]
pub const KEYWORDS: [&str; 189] = [
    // Control Flow Statements
    "if", "else", "switch", "case", "default",
    "for", "while", "do", "break", "continue",
    "goto", "return", "try", "catch", "finally",
    "throw", "throws", "loop", "match", "yield",
    "await", "async", "then", "except", "raise",
    "elif", "when", "until", "unless", "foreach",
    "in", "from", "select", "where", "defer",
    "guard", "assert", "panic", "recover",
    "next", "redo", "exit", "abort", "with",
    "elif", "end",

    // Storage Class Specifiers
    "auto", "static", "extern", "register", "typedef",
    "mutable", "constexpr", "thread_local", "let", "var",
    "const", "final", "override", "sealed", "lazy",
    "owned", "borrowed", "inline",

    // Type Qualifiers
    "const", "volatile", "restrict", "constexpr",
    "ref", "mut", "transient", "synchronized",
    "abstract", "readonly", "immutable", "dynamic",
    "weak",  "unsafe",

    // Composite Types
    "struct",  "union",  "enum",  "class",  "trait",
    "interface", "protocol" ,"record" ,"object",
    "impl" ,"concept" ,"module",

    // Miscellaneous
    "sizeof" ,"inline" ,"virtual" ,"explicit" ,"namespace",
    "using" ,"operator" ,"template" ,"typename" ,"friend",
    "crate" ,"super" ,"self" ,"import" ,"package",
    "include" ,"public" ,"private" ,"protected" ,"internal",
    "static_cast" ,"reinterpret_cast" ,"dynamic_cast" ,"const_cast",
    "typeof" ,"instanceof", "new" ,"delete" ,"clone",
    "as" ,"is" ,"extends" ,"implements" ,"default",
    "partial" ,"module" ,"export" ,"require" ,"use",

    // Data Types
    "int" ,"float" ,"double" ,"char" ,"void",
    "short" ,"long" ,"signed" ,"unsigned" ,"bool",
    "boolean" ,"byte" ,"wchar_t" ,"auto" ,"decltype",
    "nullptr_t" ,"String" ,"str" ,"u8" ,"u16",
    "u32" ,"u64" ,"u128" ,"i8" ,"i16" ,"i32",
    "i64" ,"i128" ,"f32" ,"f64" ,"usize" ,"isize",
    "any" ,"object" ,"None" ,"null" ,"undefined",
    "map" ,"list" ,"array" ,"tuple" ,"set" ,"dict",
    "Vec" ,"Option" ,"Result" ,"number" ,"char8_t",
    "char16_t" ,"char32_t" ,"interface" ,"record" ,"trait",
    "enum" ,"struct" ,"unit" ,"string" ,"symbol",
    "function" ,"object",
];
