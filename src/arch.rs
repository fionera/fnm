use crate::version::Version;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Arch {
    X86,
    X64,
    Arm64,
    Armv7l,
    Ppc64le,
    Ppc64,
    S390x,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LibC {
    Musl,
    Glibc,
}

pub trait DownloadPath {
   fn download_path(&self) -> String;
}

pub trait InstallDir {
    fn install_dir(&self) -> String;
}

#[cfg(unix)]
/// handle common case: Apple Silicon / Node < 16
pub fn get_safe_arch<'a>(arch: &'a Arch, version: &Version) -> &'a Arch {
    use crate::system_info::{platform_arch, platform_name};

    return match (platform_name(), platform_arch(), version) {
        ("darwin", "arm64", Version::Semver(v)) if v.major < 16 => &Arch::X64,
        _ => &arch,
    };
}

#[cfg(windows)]
/// handle common case: Apple Silicon / Node < 16
pub fn get_safe_arch<'a>(arch: &'a Arch, _version: &Version) -> &'a Arch {
    return &arch;
}

impl DownloadPath for LibC {
    fn download_path(&self) -> String {
        match self {
            LibC::Musl => String::from("-musl"),
            LibC::Glibc => String::from(""),
        }
    }
}

impl Default for Arch {
    fn default() -> Arch {
        match crate::system_info::platform_arch().parse() {
            Ok(arch) => arch,
            Err(e) => panic!("{}", e.details),
        }
    }
}

impl Default for LibC {
    fn default() -> LibC {
        match os_type::current_platform().os_type {
            os_type::OSType::Alpine => LibC::Musl,
            _ => LibC::Glibc,
        }
    }
}

impl std::str::FromStr for Arch {
    type Err = ArchError;
    fn from_str(s: &str) -> Result<Arch, Self::Err> {
        match s {
            "x86" => Ok(Arch::X86),
            "x64" => Ok(Arch::X64),
            "arm64" => Ok(Arch::Arm64),
            "armv7l" => Ok(Arch::Armv7l),
            "ppc64le" => Ok(Arch::Ppc64le),
            "ppc64" => Ok(Arch::Ppc64),
            "s390x" => Ok(Arch::S390x),
            unknown => Err(ArchError::new(&format!("Unknown Arch: {}", unknown))),
        }
    }
}

impl std::str::FromStr for LibC {
    type Err = LibCError;
    fn from_str(s: &str) -> Result<LibC, Self::Err> {
        match s {
            "musl" => Ok(LibC::Musl),
            "glibc" => Ok(LibC::Glibc),
            unknown => Err(LibCError::new(&format!("Unknown LibC: {}", unknown))),
        }
    }
}


impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch_str = match self {
            Arch::X86 => String::from("x86"),
            Arch::X64 => String::from("x64"),
            Arch::Arm64 => String::from("arm64"),
            Arch::Armv7l => String::from("armv7l"),
            Arch::Ppc64le => String::from("ppc64le"),
            Arch::Ppc64 => String::from("ppc64"),
            Arch::S390x => String::from("s390x"),
        };

        write!(f, "{}", arch_str)
    }
}

impl std::fmt::Display for LibC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let libc_str = match self {
            LibC::Musl => String::from("musl"),
            LibC::Glibc => String::from("glibc"),
        };

        write!(f, "{}", libc_str)
    }
}

#[derive(Debug)]
pub struct ArchError {
    details: String,
}

impl ArchError {
    fn new(msg: &str) -> ArchError {
        ArchError {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for ArchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for ArchError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct LibCError {
    details: String,
}

impl LibCError {
    fn new(msg: &str) -> LibCError {
        LibCError {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for LibCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for LibCError {
    fn description(&self) -> &str {
        &self.details
    }
}