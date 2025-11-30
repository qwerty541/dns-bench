use crate::args::Format;
use crate::args::IpAddr;
use crate::args::Protocol;
use crate::args::Style;
use crate::cli::SharedArgs;

use clap::ValueEnum;
use directories::UserDirs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

const CONFIG_DIR_NAME: &str = ".dns-bench";
const CONFIG_FILE_NAME: &str = "config.toml";
const USER_DIRS_ERROR: &str =
    "No valid home directory path could be retrieved from the operating system.";

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DnsBenchConfig {
    pub domain: String,
    pub threads: u16,
    pub requests: u16,
    pub timeout: u64,
    pub protocol: Protocol,
    pub name_servers_ip: IpAddr,
    pub lookup_ip: IpAddr,
    pub style: Style,
    pub custom_servers_file: Option<PathBuf>,
    #[serde(default)]
    pub format: Format,
    #[serde(default)]
    pub skip_system_servers: bool,
    #[serde(default)]
    pub skip_gateway_detection: bool,
    // WARNING! Addition of the serde default attribute for all new fields is important to ensure backward compatibility
    // with older configuration files that may not have these fields defined.
}

impl Default for DnsBenchConfig {
    fn default() -> Self {
        DnsBenchConfig {
            domain: String::from("google.com"),
            threads: 8,
            requests: 25,
            timeout: 1,
            protocol: Protocol::Udp,
            name_servers_ip: IpAddr::V4,
            lookup_ip: IpAddr::V4,
            style: Style::Rounded,
            custom_servers_file: None,
            format: Format::HumanReadable,
            skip_system_servers: false,
            skip_gateway_detection: false,
        }
    }
}

impl DnsBenchConfig {
    pub fn resolve_args(&mut self, args: &SharedArgs) {
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
        if let Some(custom_servers_file) = &args.custom_servers_file {
            self.custom_servers_file = fs::canonicalize(custom_servers_file).ok()
        }
        if let Some(format) = args.format {
            self.format = format;
        }
        if args.skip_system_servers {
            self.skip_system_servers = true;
        }
        if args.skip_gateway_detection {
            self.skip_gateway_detection = true;
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

    pub fn config_file_path() -> Result<PathBuf, Box<dyn Error>> {
        let user_dirs = UserDirs::new().ok_or(USER_DIRS_ERROR)?;
        let home_dir = user_dirs.home_dir().to_path_buf();
        Ok(home_dir.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
    }

    pub fn config_file_exists() -> Result<bool, Box<dyn Error>> {
        Ok(Self::config_file_path()?.exists())
    }

    pub fn delete_config_file() -> Result<(), Box<dyn Error>> {
        let path = Self::config_file_path()?;
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

impl fmt::Display for DnsBenchConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "domain: {}", self.domain)?;
        writeln!(f, "threads: {}", self.threads)?;
        writeln!(f, "requests: {}", self.requests)?;
        writeln!(f, "timeout: {}", self.timeout)?;
        writeln!(
            f,
            "protocol: {}",
            self.protocol
                .to_possible_value()
                .expect("Failed to get protocol name")
                .get_name()
        )?;
        writeln!(
            f,
            "name-servers-ip: {}",
            self.name_servers_ip
                .to_possible_value()
                .expect("Failed to get name servers IP")
                .get_name()
        )?;
        writeln!(
            f,
            "lookup-ip: {}",
            self.lookup_ip
                .to_possible_value()
                .expect("Failed to get lookup IP")
                .get_name()
        )?;
        writeln!(
            f,
            "style: {}",
            self.style
                .to_possible_value()
                .expect("Failed to get style")
                .get_name()
        )?;

        if let Some(custom_servers_file) = &self.custom_servers_file {
            writeln!(f, "custom-servers-file: {}", custom_servers_file.display())?;
        } else {
            writeln!(f, "custom-servers-file: null")?; // Explicitly show null if not set
        }

        writeln!(
            f,
            "format: {}",
            self.format
                .to_possible_value()
                .expect("Failed to get format")
                .get_name()
        )?;
        writeln!(f, "skip-system-servers: {}", self.skip_system_servers)?;
        writeln!(f, "skip-gateway-detection: {}", self.skip_gateway_detection)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum LoadConfigResult {
    Loaded(DnsBenchConfig),
    FileDoesNotExist,
    Error(LoadConfigError),
}

impl LoadConfigResult {
    pub fn unwrap_or_default(self) -> DnsBenchConfig {
        match self {
            LoadConfigResult::Loaded(c) => c,
            LoadConfigResult::FileDoesNotExist => DnsBenchConfig::default(),
            LoadConfigResult::Error(e) => {
                eprintln!(
                    "Failed to load config: {e:?}\n\
                    Proceeding with default parameters..."
                );
                DnsBenchConfig::default()
            }
        }
    }
}

#[derive(Debug, derive_more::Error, derive_more::From)]
pub enum LoadConfigError {
    UserDirs,
    Io(io::Error),
    Toml(toml::de::Error),
}

impl fmt::Display for LoadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadConfigError::UserDirs => write!(f, "UserDirs: {USER_DIRS_ERROR}"),
            LoadConfigError::Io(e) => write!(f, "Io: {e}"),
            LoadConfigError::Toml(e) => write!(f, "Toml: {e}"),
        }
    }
}
