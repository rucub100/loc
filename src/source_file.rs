use std::path::{Path, PathBuf};

#[derive(Debug)]
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
            "java" => Some(ProgrammingLanguage::Java),
            _ => None,
        }
    }
}

pub struct SourceFile {
    path: PathBuf,
    lang: ProgrammingLanguage,
}

impl SourceFile {
    pub fn from_path(path: PathBuf) -> Option<SourceFile> {
        assert!(path.is_file());
        let extension = path.extension()?.to_str()?;
        let lang = ProgrammingLanguage::from_extension(extension)?;

        Some(SourceFile { path, lang })
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_lang(&self) -> &ProgrammingLanguage {
        &self.lang
    }

    pub fn loc(&self) -> u32 {
        // TODO
        0
    }
}
