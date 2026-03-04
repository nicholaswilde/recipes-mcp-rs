use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "RECIPES_LOG_LEVEL")]
    pub log_level: Option<String>,

    #[arg(short, long, env = "RECIPES_MCP_TRANSPORT")]
    pub mcp_transport: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub log_level: String,
    pub mcp_transport: String,
}

impl AppConfig {
    pub fn load(args: Args) -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .set_default("log_level", "info")?
            .set_default("mcp_transport", "stdio")?
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("RECIPES"));

        // CLI Overrides
        if let Some(level) = args.log_level {
            builder = builder.set_override("log_level", level)?;
        }
        if let Some(transport) = args.mcp_transport {
            builder = builder.set_override("mcp_transport", transport)?;
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
            env::remove_var("RECIPES_LOG_LEVEL");
            env::remove_var("RECIPES_MCP_TRANSPORT");
        }

        let args = Args {
            log_level: None,
            mcp_transport: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.mcp_transport, "stdio");
    }

    #[test]
    #[serial]
    fn test_env_override() {
        unsafe {
            env::set_var("RECIPES_LOG_LEVEL", "debug");
        }
        let args = Args {
            log_level: None,
            mcp_transport: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "debug");
        unsafe {
            env::remove_var("RECIPES_LOG_LEVEL");
        }
    }

    #[test]
    #[serial]
    fn test_cli_override() {
        unsafe {
            env::set_var("RECIPES_LOG_LEVEL", "info");
        }
        let args = Args {
            log_level: Some("trace".into()),
            mcp_transport: None,
        };
        let config = AppConfig::load(args).unwrap();
        assert_eq!(config.log_level, "trace");
        unsafe {
            env::remove_var("RECIPES_LOG_LEVEL");
        }
    }
}
