use crate::args::Arguments;
use crate::args::IpAddr;
use crate::args::Protocol;
use crate::args::Style;

use directories::UserDirs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

const CONFIG_DIR_NAME: &str = ".dns-bench";
const CONFIG_FILE_NAME: &str = "config.toml";
const USER_DIRS_ERROR: &str =
    "No valid home directory path could be retrieved from the operating system.";

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DnsBenchConfig {
    pub domain: String,
    pub threads: usize,
    pub requests: usize,
    pub timeout: u64,
    pub protocol: Protocol,
    pub name_servers_ip: IpAddr,
    pub lookup_ip: IpAddr,
    pub style: Style,
}

impl Default for DnsBenchConfig {
    fn default() -> Self {
        DnsBenchConfig {
            domain: String::from("google.com"),
            threads: 8,
            requests: 3,
            timeout: 3,
            protocol: Protocol::Udp,
            name_servers_ip: IpAddr::V4,
            lookup_ip: IpAddr::V4,
            style: Style::Ascii,
        }
    }
}

impl DnsBenchConfig {
    pub fn resolve_args(&mut self, args: &Arguments) {
        if let Some(domain) = &args.domain {
            self.domain.clone_from(domain);
        }
        if let Some(threads) = args.threads {
            self.threads = threads;
        }
        if let Some(requests) = args.requests {
            self.requests = requests;
        }
        if let Some(timeout) = args.timeout {
            self.timeout = timeout;
        }
        if let Some(protocol) = args.protocol {
            self.protocol = protocol;
        }
        if let Some(name_servers_ip) = args.name_servers_ip {
            self.name_servers_ip = name_servers_ip;
        }
        if let Some(lookup_ip) = args.lookup_ip {
            self.lookup_ip = lookup_ip;
        }
        if let Some(style) = args.style {
            self.style = style;
        }
    }

    pub fn try_load_from_file() -> LoadConfigResult {
        let Some(user_dirs) = UserDirs::new() else {
            return LoadConfigResult::Error(LoadConfigError::UserDirs);
        };
        let home_dir = user_dirs.home_dir().to_path_buf();
        let config_path = home_dir.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME);

        if !config_path.exists() {
            return LoadConfigResult::FileDoesNotExist;
        }

        let config_str = match fs::read_to_string(&config_path) {
            Ok(s) => s,
            Err(e) => return LoadConfigResult::Error(LoadConfigError::Io(e)),
        };

        let config: DnsBenchConfig = match toml::from_str(&config_str) {
            Ok(c) => c,
            Err(e) => return LoadConfigResult::Error(LoadConfigError::Toml(e)),
        };

        LoadConfigResult::Loaded(config)
    }

    pub fn write_into_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_dirs = UserDirs::new().ok_or(USER_DIRS_ERROR)?;
        let home_dir = user_dirs.home_dir().to_path_buf();
        let config_dir = home_dir.join(CONFIG_DIR_NAME);

        fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join(CONFIG_FILE_NAME);
        let config_str = toml::to_string_pretty(self)?;

        fs::write(config_path, config_str)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum LoadConfigResult {
    Loaded(DnsBenchConfig),
    FileDoesNotExist,
    Error(LoadConfigError),
}

#[derive(Debug)]
pub enum LoadConfigError {
    UserDirs,
    Io(io::Error),
    Toml(toml::de::Error),
}

impl fmt::Display for LoadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadConfigError::UserDirs => write!(f, "UserDirs: {}", USER_DIRS_ERROR),
            LoadConfigError::Io(e) => write!(f, "Io: {}", e),
            LoadConfigError::Toml(e) => write!(f, "Toml: {}", e),
        }
    }
}

impl From<io::Error> for LoadConfigError {
    fn from(e: io::Error) -> Self {
        LoadConfigError::Io(e)
    }
}

impl From<toml::de::Error> for LoadConfigError {
    fn from(e: toml::de::Error) -> Self {
        LoadConfigError::Toml(e)
    }
}

impl Error for LoadConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadConfigError::UserDirs => None,
            LoadConfigError::Io(e) => Some(e),
            LoadConfigError::Toml(e) => Some(e),
        }
    }
}
