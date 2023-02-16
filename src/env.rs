use thiserror::Error;

#[derive(Clone)]
pub struct Env {
    pub server_path: String,
    pub run_command: String,
    pub rcon_password: String,
    pub address_hint: String,
    pub webserver_port: u16,
    pub minecraft_port: u16,
    pub rcon_port: u16,
    pub minecraft_idle_timeout: u64,
}

/// Ensure all environment variables are present and that the non-string values are parsable.
/// This is so we can safely unwrap when retrieving them later.
pub fn load_env() -> Result<Env, EnvError> {
    use EnvError::*;

    Ok(Env {
        server_path: std::env::var("SERVER_PATH")
            .map_err(|_| ServerPath)?,

        run_command: std::env::var("RUN_COMMAND")
            .map_err(|_| RunCommand)?,
        
        rcon_password: std::env::var("RCON_PASSWORD")
            .map_err(|_| RconPassword)?,
        
        address_hint: std::env::var("ADDRESS_HINT")
            .map_err(|_| AddressHint)?,
        
        webserver_port: std::env::var("WEBSERVER_PORT")
            .map_err(|_| WebserverPort)?
            .parse()
            .map_err(|_| WebserverPortValue)?,
        
        minecraft_port: std::env::var("MINECRAFT_PORT")
            .map_err(|_| MinecraftPort)?
            .parse()
            .map_err(|_| MinecraftPortValue)?,
        
        rcon_port: std::env::var("RCON_PORT")
            .map_err(|_| RconPort)?
            .parse()
            .map_err(|_| RconPortValue)?,
        
        minecraft_idle_timeout: std::env::var("MINECRAFT_IDLE_TIMEOUT")
            .map_err(|_| MinecraftIdleTimeout)?
            .parse()
            .map_err(|_| MinecraftIdleTimeoutValue)?,
    })
}

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Missing the SERVER_PATH configuration variable")]
    ServerPath,
    #[error("Missing the RUN_COMMAND configuration variable")]
    RunCommand,
    #[error("Missing the RCON_PASSWORD configuration variable")]
    RconPassword,
    #[error("Missing the ADDRESS_HINT configuration variable")]
    AddressHint,
    #[error("Missing the WEBSERVER_PORT configuration variable")]
    WebserverPort,
    #[error("Missing the MINECRAFT_PORT configuration variable")]
    MinecraftPort,
    #[error("Missing the RCON_PORT configuration variable")]
    RconPort,
    #[error("Missing the MINECRAFT_IDLE_TIMEOUT configuration variable")]
    MinecraftIdleTimeout,

    #[error("WEBSERVER_PORT is not a valid 16-bit unsigned integer")]
    WebserverPortValue,
    #[error("MINECRAFT_PORT is not a valid 16-bit unsigned integer")]
    MinecraftPortValue,
    #[error("RCON_PORT is not a valid 16-bit unsigned integer")]
    RconPortValue,
    #[error("MINECRAFT_IDLE_TIMEOUT is not a valid 64-bit unsigned integer")]
    MinecraftIdleTimeoutValue,
}
