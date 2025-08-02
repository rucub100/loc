use std::path::PathBuf;

pub enum ProgrammingLanguage {
    Ada,
    Assembly,
    C,
    COBOL,
    CPlusPlus,
    CSharp,
    Closure,
    Crystal,
    D,
    Dart,
    Elixir,
    Erlang,
    FSharp,
    Fortran,
    Go,
    Groovy,
    Haskell,
    Java,
    JavaScript,
    Julia,
    Kotlin,
    Lisp,
    Lua,
    MATLAB,
    OCaml,
    ObjectiveC,
    PHP,
    Perl,
    PowerShell,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    ShellBash,
    Smalltalk,
    Swift,
    TypeScript,
    VisualBasic,
    Zig,
}

// TODO
pub struct SourceFile {
    path: PathBuf,
    lang: ProgrammingLanguage,
}

// TODO: GitIgnore
// https://github.com/github/gitignore

impl ProgrammingLanguage {
    pub const fn well_known_extensions(&self) -> Option<&'static [&'static str]> {
        match self {
            ProgrammingLanguage::C => Some(&["c", "h"]),
            ProgrammingLanguage::CPlusPlus => Some(&["cpp", "cc", "cxx", "hpp", "hxx", "hh", "h"]),
            ProgrammingLanguage::CSharp => Some(&["cs", "csx"]),
            ProgrammingLanguage::Go => Some(&["go"]),
            ProgrammingLanguage::Java => Some(&["java"]),
            ProgrammingLanguage::JavaScript => Some(&["js", "mjs", "cjs"]),
            ProgrammingLanguage::PHP => Some(&["php"]),
            ProgrammingLanguage::Python => Some(&["py"]),
            ProgrammingLanguage::Rust => Some(&["rs"]),
            ProgrammingLanguage::TypeScript => Some(&["ts", "tsx"]),
            _ => None,
        }
    }

    pub fn from_extension(extension: &str) -> Option<ProgrammingLanguage> {
        match extension {
            "cs" | "csx" => Some(ProgrammingLanguage::CSharp),
            "js" | "mjs" | "cjs" => Some(ProgrammingLanguage::JavaScript),
            "rs" => Some(ProgrammingLanguage::Rust),
            "ts" => Some(ProgrammingLanguage::TypeScript),
            _ => None,
        }
    }
}
