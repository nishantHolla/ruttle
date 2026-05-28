use regex::Regex;

// Values

pub const PART_EXTENSION: &str = ".part.html";
pub const OUTPUT_EXTENSION: &str = ".html";

// Directives

pub const DEFINE_DIRECTIVE_START: &str = "{#define";
pub const INTERPOLATE_DIRECTIVE_START: &str = "{#value";
pub const INCLUDE_DIRECTIVE_START: &str = "{#include";
pub const WITH_DIRECTIVE_START: &str = "{#with";
pub const FOR_DIRECTIVE_START: &str = "{#for";
pub const IF_DIRECTIVE_START: &str = "{#if";
pub const ONCE_DIRECTIVE_START: &str = "{#once";
pub const DIRECTIVE_END: &str = "}";

pub const DIRECTIVE_REGEX: &str = r"\{#(define|value|include|with|for|if|once)\b";
pub const FOR_DIRECTIVE_REGEX: &str = r"(?<l>\w+)[\s\n]*,[\s\n]*(?<r>\w+)[\s\n]+(?<keyword>\w+)[\s\n]+(?:(?<range>[+\-]?\d+\.\.[+\-]?\d+\.\.[+\-]?\d+)|(?<path>\S+))[\s\n]+(?<body>[\s\S]*)$";
pub const INTERPOLATE_DIRECTIVE_REGEX: &str = r"\{#value[\s\n]+([\w.]+)[\s\n]*\}";

pub static DIRECTIVE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(DIRECTIVE_REGEX).unwrap());

pub static FOR_DIRECTIVE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(FOR_DIRECTIVE_REGEX).unwrap());

pub static INTERPOLATE_DIRECTIVE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(INTERPOLATE_DIRECTIVE_REGEX).unwrap());

// Language

pub const KV_SPLIT: &str = "=";
