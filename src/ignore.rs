use std::collections::HashSet;

pub struct Ignore {
    dirs: HashSet<&'static str>,
}

impl Ignore {
    pub fn new() -> Ignore {
        let mut dirs = HashSet::new();

        // https://github.com/github/gitignore
        // only lowercase (to lowercase)
        dirs.insert(".cache");
        dirs.insert(".docusaurus");
        dirs.insert(".dynamodb");
        dirs.insert(".firebase");
        dirs.insert(".fusebox");
        dirs.insert(".git");
        dirs.insert(".gradle");
        dirs.insert(".idea");
        dirs.insert(".mtj.tmp");
        dirs.insert(".mvn");
        dirs.insert(".next");
        dirs.insert(".npm");
        dirs.insert(".nuxt");
        dirs.insert(".serverless");
        dirs.insert(".svelte-kit");
        dirs.insert(".temp");
        dirs.insert(".vscode");
        dirs.insert(".yarn");
        dirs.insert("arm");
        dirs.insert("arm64");
        dirs.insert("artifacts");
        dirs.insert("bin");
        dirs.insert("bld");
        dirs.insert("bower_components");
        dirs.insert("build");
        dirs.insert("codecoverage");
        dirs.insert("coverage");
        dirs.insert("debug");
        dirs.insert("debugpublic");
        dirs.insert("dist");
        dirs.insert("jspm_packages");
        dirs.insert("lib-cov");
        dirs.insert("log");
        dirs.insert("logs");
        dirs.insert("node_modules");
        dirs.insert("obj");
        dirs.insert("out");
        dirs.insert("release");
        dirs.insert("releases");
        dirs.insert("target");
        dirs.insert("web_modules");
        dirs.insert("win32");
        dirs.insert("x64");
        dirs.insert("x86");

        Ignore { dirs }
    }

    pub fn should_ignore_dir(&self, name: &str) -> bool {
        self.dirs.contains(name.to_lowercase().as_str())
    }
}
