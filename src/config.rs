use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "RECIPES__LOG_LEVEL")]
    pub log_level: Option<String>,

    #[arg(short, long, env = "RECIPES__TRANSPORT")]
    pub transport: Option<String>,

    #[arg(short, long, env = "RECIPES__PORT")]
    pub port: Option<u16>,

    #[arg(long, env = "RECIPES__WEIGHT_CONVERSION", action = clap::ArgAction::Set)]
    pub weight_conversion: Option<bool>,

    #[arg(long, env = "RECIPES__CACHE_ENABLED", action = clap::ArgAction::Set)]
    pub cache_enabled: Option<bool>,

    #[arg(long, env = "RECIPES__CACHE_DIR")]
    pub cache_dir: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub log_level: String,
    pub transport: String,
    pub port: u16,
    pub weight_conversion: bool,
    pub cache_enabled: bool,
    pub cache_dir: String,
}

impl AppConfig {
    pub fn load(args: Args) -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .set_default("log_level", "info")?
            .set_default("transport", "stdio")?
            .set_default("port", 3000)?
            .set_default("weight_conversion", true)?
            .set_default("cache_enabled", true)?
            .set_default("cache_dir", ".cache")?
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("RECIPES").separator("__"));

        // CLI Overrides
        if let Some(level) = args.log_level {
            builder = builder.set_override("log_level", level)?;
        }
        if let Some(transport) = args.transport {
            builder = builder.set_override("transport", transport)?;
        }
        if let Some(port) = args.port {
            builder = builder.set_override("port", port)?;
        }
        if let Some(weight_conv) = args.weight_conversion {
            builder = builder.set_override("weight_conversion", weight_conv)?;
        }
        if let Some(cache_enabled) = args.cache_enabled {
            builder = builder.set_override("cache_enabled", cache_enabled)?;
        }
        if let Some(cache_dir) = args.cache_dir {
            builder = builder.set_override("cache_dir", cache_dir)?;
        }

        let s = builder.build()?;
        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn test_default_config() {
        unsafe {
            env::remove_var("RECIPES__LOG_LEVEL");
            env::remove_var("RECIPES__TRANSPORT");
            env::remove_var("RECIPES__PORT");
            env::remove_var("RECIPES__WEIGHT_CONVERSION");
            env::remove_var("RECIPES__CACHE_ENABLED");
            env::remove_var("RECIPES__CACHE_DIR");
        }

        let args = Args {
            log_level: None,
            transport: None,
            port: None,
            weight_conversion: None,
            cache_enabled: None,
            cache_dir: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.transport, "stdio");
        assert_eq!(config.port, 3000);
        assert!(config.weight_conversion);
        assert!(config.cache_enabled);
        assert_eq!(config.cache_dir, ".cache");
    }

    #[test]
    #[serial]
    fn test_env_override() {
        unsafe {
            env::set_var("RECIPES__LOG_LEVEL", "debug");
            env::set_var("RECIPES__PORT", "4000");
            env::set_var("RECIPES__WEIGHT_CONVERSION", "false");
            env::set_var("RECIPES__CACHE_ENABLED", "false");
            env::set_var("RECIPES__CACHE_DIR", "/tmp/cache");
        }
        let args = Args {
            log_level: None,
            transport: None,
            port: None,
            weight_conversion: None,
            cache_enabled: None,
            cache_dir: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.port, 4000);
        assert!(!config.weight_conversion);
        assert!(!config.cache_enabled);
        assert_eq!(config.cache_dir, "/tmp/cache");
        unsafe {
            env::remove_var("RECIPES__LOG_LEVEL");
            env::remove_var("RECIPES__PORT");
            env::remove_var("RECIPES__WEIGHT_CONVERSION");
            env::remove_var("RECIPES__CACHE_ENABLED");
            env::remove_var("RECIPES__CACHE_DIR");
        }
    }

    #[test]
    #[serial]
    fn test_cli_override() {
        unsafe {
            env::set_var("RECIPES__LOG_LEVEL", "info");
            env::set_var("RECIPES__PORT", "3000");
            env::set_var("RECIPES__WEIGHT_CONVERSION", "true");
        }
        let args = Args {
            log_level: Some("trace".into()),
            transport: None,
            port: Some(5000),
            weight_conversion: Some(false),
            cache_enabled: None,
            cache_dir: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "trace");
        assert_eq!(config.port, 5000);
        assert!(!config.weight_conversion);
        unsafe {
            env::remove_var("RECIPES__LOG_LEVEL");
            env::remove_var("RECIPES__PORT");
            env::remove_var("RECIPES__WEIGHT_CONVERSION");
        }
    }

    #[test]
    #[serial]
    fn test_port_config() {
        unsafe {
            env::remove_var("RECIPES__PORT");
        }
        let args = Args {
            log_level: None,
            transport: None,
            port: Some(8080),
            weight_conversion: None,
            cache_enabled: None,
            cache_dir: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.port, 8080);
        
        let args_default = Args {
            log_level: None,
            transport: None,
            port: None,
            weight_conversion: None,
            cache_enabled: None,
            cache_dir: None,
        };
        let config_default = AppConfig::load(args_default).unwrap();
        assert_eq!(config_default.port, 3000);
    }
}
