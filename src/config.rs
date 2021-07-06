use dirs::home_dir;
use reqwest::Url;
use structopt::lazy_static::lazy_static;
use structopt::StructOpt;

use crate::arch;
use crate::arch::LibC;
use crate::log_level::LogLevel;
use crate::path_ext::PathExt;

#[derive(StructOpt, Debug)]
pub struct FnmConfig {
    /// https://nodejs.org/dist/ mirror.
    /// When using musl set it to https://unofficial-builds.nodejs.org/download/release/
    #[structopt(
    long,
    env = "FNM_NODE_DIST_MIRROR",
    default_value = &DEFAULT_NODE_DIST_MIRROR,
    global = true,
    hide_env_values = true
    )]
    pub node_dist_mirror: reqwest::Url,

    /// The root directory of fnm installations.
    #[structopt(
    long = "fnm-dir",
    env = "FNM_DIR",
    global = true,
    hide_env_values = true
    )]
    pub base_dir: Option<std::path::PathBuf>,

    /// Where the current node version link is stored.
    /// This value will be populated automatically by evaluating
    /// `fnm env` in your shell profile. Read more about it using `fnm help env`
    #[structopt(
    long,
    env = "FNM_MULTISHELL_PATH",
    hide_env_values = true,
    hidden = true
    )]
    multishell_path: Option<std::path::PathBuf>,

    /// The log level of fnm commands
    #[structopt(
    long,
    env = "FNM_LOGLEVEL",
    default_value = "info",
    global = true,
    hide_env_values = true
    )]
    log_level: LogLevel,

    /// Override the architecture of the installed Node binary.
    /// Defaults to arch of fnm binary.
    #[structopt(
    long,
    env = "FNM_ARCH",
    default_value,
    global = true,
    hide_env_values = true
    )]
    pub arch: arch::Arch,

    /// Override the libc of the installed Node binary.
    /// Defaults to musl if on alpine.
    /// Only used if arch is not windows.
    #[structopt(
    long,
    env = "FNM_LIBC",
    default_value,
    global = true,
    hide_env_values = true
    )]
    pub libc: arch::LibC,
}

lazy_static! {
    pub static ref DEFAULT_NODE_DIST_MIRROR: String = default_node_dist_mirror();
}

fn default_node_dist_mirror() -> String {
    (match LibC::default() {
        LibC::Glibc => "https://nodejs.org/dist/",
        LibC::Musl => "https://unofficial-builds.nodejs.org/download/release/",
    }).to_string()
}

impl Default for FnmConfig {
    fn default() -> Self {
        Self {
            node_dist_mirror: reqwest::Url::parse(default_node_dist_mirror().as_str()).unwrap(),
            base_dir: None,
            multishell_path: None,
            log_level: LogLevel::Info,
            arch: Default::default(),
            libc: Default::default(),
        }
    }
}

impl FnmConfig {
    pub fn multishell_path(&self) -> Option<&std::path::Path> {
        match &self.multishell_path {
            None => None,
            Some(v) => Some(v.as_path()),
        }
    }

    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    pub fn base_dir_with_default(&self) -> std::path::PathBuf {
        self.base_dir
            .clone()
            .unwrap_or_else(|| home_dir().expect("Can't get home directory").join(".fnm"))
            .ensure_exists_silently()
    }

    pub fn installations_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("node-versions")
            .ensure_exists_silently()
    }

    pub fn default_version_dir(&self) -> std::path::PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("aliases")
            .ensure_exists_silently()
    }

    #[cfg(test)]
    pub fn with_base_dir(mut self, base_dir: Option<std::path::PathBuf>) -> Self {
        self.base_dir = base_dir;
        self
    }

    pub fn multishell_base_dir(&self) -> std::path::PathBuf {
        std::env::temp_dir()
            .join("fnm_multishell")
            .ensure_exists_silently()
    }
}
