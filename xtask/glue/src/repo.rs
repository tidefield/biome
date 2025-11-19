use crate::project_root;
use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

// Lock in version of the repos used across the project
const BABEL_REVISION: &str = "33a6be4e56b149647c15fd6c0157c1413456851d";
const TYPESCRIPT_REVISION: &str = "61a96b1641abe24c4adc3633eb936df89eb991f2";
const TEST262_REVISION: &str = "715dd1073bc060f4ee221e2e74770f5728e7b8a0";

const BABEL_URL: &str = "https://github.com/babel/babel.git";
const TYPESCRIPT_URL: &str = "https://github.com/microsoft/Typescript.git";
const TEST262_URL: &str = "https://github.com/tc39/test262.git";

pub struct Repo {
    name: RepoName,
    depth: Option<u32>,
}

#[derive(Copy, Clone)]
pub enum RepoName {
    Babel,
    Typescript,
    Test262,
}

impl RepoName {
    pub fn as_str(&self) -> &str {
        match self {
            RepoName::Babel => "babel",
            RepoName::Typescript => "Typescript",
            RepoName::Test262 => "test262",
        }
    }

    pub fn checkout_dir(self) -> &'static Path {
        static PATH: OnceLock<PathBuf> = OnceLock::new();
        PATH.get_or_init(|| project_root().join("xtask/glue").join(self.as_str()))
            .as_path()
    }
}

impl Repo {
    pub fn revision(&self) -> &str {
        match self.name {
            RepoName::Babel => BABEL_REVISION,
            RepoName::Typescript => TYPESCRIPT_REVISION,
            RepoName::Test262 => TEST262_REVISION,
        }
    }

    pub fn url(&self) -> &str {
        match self.name {
            RepoName::Babel => BABEL_URL,
            RepoName::Typescript => TYPESCRIPT_URL,
            RepoName::Test262 => TEST262_URL,
        }
    }
}

impl Repo {
    pub fn new(name: RepoName, depth: Option<u32>) -> Self {
        Self { name, depth }
    }

    pub fn exists(&self) -> bool {
        self.name.checkout_dir().exists()
    }

    pub fn checkout(&self) -> io::Result<()> {
        if !self.exists() {
            let mut command = Command::new("git");
            command
                .arg("clone")
                .arg(self.url())
                .arg(&self.name.checkout_dir())
                .arg("--revision")
                .arg(self.revision());
            if let Some(depth) = self.depth {
                command.arg("--depth").arg(depth.to_string());
            }
            let output = command.output()?;
            if !output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "failed to clone {}: {}",
                        self.url(),
                        String::from_utf8_lossy(&output.stderr)
                    ),
                ));
            }
        }
        Ok(())
    }
}
