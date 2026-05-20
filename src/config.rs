pub const PART_EXTENSION: &str = ".part.html";
pub const OUTPUT_EXTENSION: &str = ".html";

pub const DEFINE_DIRECTIVE_START: &str = "{#define";
pub const INTERPOLATE_DIRECTIVE_START: &str = "{#value";
pub const INCLUDE_DIRECTIVE_START: &str = "{#include";
pub const DIRECTIVE_REGEX: &str = r"\{#(define|value|include)\b";
pub const KV_SPLIT: &str = "=";
pub const DIRECTIVE_END: &str = "}";
