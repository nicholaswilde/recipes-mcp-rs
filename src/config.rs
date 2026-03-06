use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "RECIPES__LOG_LEVEL")]
    pub log_level: Option<String>,

    #[arg(short, long, env = "RECIPES__MCP_TRANSPORT")]
    pub mcp_transport: Option<String>,

    #[arg(short, long, env = "RECIPES__PORT")]
    pub port: Option<u16>,

    #[arg(long, env = "RECIPES__WEIGHT_CONVERSION", action = clap::ArgAction::Set)]
    pub weight_conversion: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub log_level: String,
    pub mcp_transport: String,
    pub port: u16,
    pub weight_conversion: bool,
}

impl AppConfig {
    pub fn load(args: Args) -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .set_default("log_level", "info")?
            .set_default("mcp_transport", "stdio")?
            .set_default("port", 3000)?
            .set_default("weight_conversion", true)?
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("RECIPES").separator("__"));

        // CLI Overrides
        if let Some(level) = args.log_level {
            builder = builder.set_override("log_level", level)?;
        }
        if let Some(transport) = args.mcp_transport {
            builder = builder.set_override("mcp_transport", transport)?;
        }
        if let Some(port) = args.port {
            builder = builder.set_override("port", port)?;
        }
        if let Some(weight_conv) = args.weight_conversion {
            builder = builder.set_override("weight_conversion", weight_conv)?;
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
            env::remove_var("RECIPES__MCP_TRANSPORT");
            env::remove_var("RECIPES__PORT");
            env::remove_var("RECIPES__WEIGHT_CONVERSION");
        }

        let args = Args {
            log_level: None,
            mcp_transport: None,
            port: None,
            weight_conversion: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.mcp_transport, "stdio");
        assert_eq!(config.port, 3000);
        assert!(config.weight_conversion);
    }

    #[test]
    #[serial]
    fn test_env_override() {
        unsafe {
            env::set_var("RECIPES__LOG_LEVEL", "debug");
            env::set_var("RECIPES__PORT", "4000");
            env::set_var("RECIPES__WEIGHT_CONVERSION", "false");
        }
        let args = Args {
            log_level: None,
            mcp_transport: None,
            port: None,
            weight_conversion: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.port, 4000);
        assert!(!config.weight_conversion);
        unsafe {
            env::remove_var("RECIPES__LOG_LEVEL");
            env::remove_var("RECIPES__PORT");
            env::remove_var("RECIPES__WEIGHT_CONVERSION");
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
            mcp_transport: None,
            port: Some(5000),
            weight_conversion: Some(false),
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
            mcp_transport: None,
            port: Some(8080),
            weight_conversion: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.port, 8080);
        
        let args_default = Args {
            log_level: None,
            mcp_transport: None,
            port: None,
            weight_conversion: None,
        };
        let config_default = AppConfig::load(args_default).unwrap();
        assert_eq!(config_default.port, 3000);
    }
}
