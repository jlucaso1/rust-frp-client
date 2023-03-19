use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::frpc::FrpcProps;

#[derive(Debug, Clone)]
pub struct Proxy {
    pub server_addr: String,
    pub server_port: u16,
    pub proxy_type: String,
}

#[derive(Debug, Clone)]
pub struct ClientCommonConfig {
    server_addr: String,
    server_port: u16,
    token: String,
}

impl ClientCommonConfig {
    pub fn new() -> ClientCommonConfig {
        ClientCommonConfig {
            server_addr: "127.0.0.1".to_string(),
            server_port: 7000,

            token: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientTcpConfig {
    pub service_type: String,
    local_ip: String,
    local_port: u16,
    pub remote_port: u16,
}

impl ClientTcpConfig {
    pub fn new() -> ClientTcpConfig {
        ClientTcpConfig {
            service_type: "tcp".to_string(),
            local_ip: "127.0.0.1".to_string(),
            local_port: 0,
            remote_port: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientWebConfig {
    pub service_type: String,
    local_ip: String,
    local_port: u16,
    pub custom_domains: Option<String>,
    pub subdomain: Option<String>,
}

impl ClientWebConfig {
    pub fn new(stype: String) -> ClientWebConfig {
        ClientWebConfig {
            service_type: stype,
            local_ip: "127.0.0.1".to_string(),
            local_port: 0,
            custom_domains: None,
            subdomain: None,
        }
    }

    pub fn check(&self) -> bool {
        if self.custom_domains.is_none() && self.subdomain.is_none() {
            return false;
        }

        return true;
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    common: ClientCommonConfig,
    pub tcp_configs: HashMap<String, ClientTcpConfig>,
    pub web_configs: HashMap<String, ClientWebConfig>,
}

impl Config {
    pub fn new() -> Self {
        let common: ClientCommonConfig = ClientCommonConfig::new();
        let tcp_configs: HashMap<String, ClientTcpConfig> = HashMap::new();
        let web_configs: HashMap<String, ClientWebConfig> = HashMap::new();

        Self {
            common,
            tcp_configs,
            web_configs,
        }
    }

    pub fn load_config(&mut self, frpc_props: &FrpcProps) -> Result<()> {
        self.parse_common_config(&frpc_props).unwrap();
        self.parse_proxy_config(&frpc_props).unwrap();

        Ok(())
    }

    pub fn server_addr(&self) -> &str {
        &self.common.server_addr
    }

    pub fn server_port(&self) -> u16 {
        self.common.server_port
    }

    pub fn auth_token(&self) -> &str {
        &self.common.token
    }

    pub fn get_proxy(&self, proxy_name: &str) -> Result<Proxy> {
        if self.tcp_configs.contains_key(proxy_name) {
            let config = self.tcp_configs.get(proxy_name).unwrap();

            Ok(Proxy {
                server_addr: config.local_ip.clone(),
                server_port: config.local_port,
                proxy_type: "tcp".to_string(),
            })
        } else if self.web_configs.contains_key(proxy_name) {
            let config = self.web_configs.get(proxy_name).unwrap();

            Ok(Proxy {
                server_addr: config.local_ip.clone(),
                server_port: config.local_port,
                proxy_type: "web".to_string(),
            })
        } else {
            Err(anyhow!("no such proxy"))
        }
    }

    fn parse_common_config(&mut self, frpc_props: &FrpcProps) -> Result<()> {
        self.common.server_addr = frpc_props.remote_addr.to_string();
        self.common.server_port = frpc_props.server_port;

        Ok(())
    }

    fn parse_proxy_config(&mut self, frpc_props: &FrpcProps) -> Result<()> {
        let mut tcp_proxy_config = ClientTcpConfig::new();
        tcp_proxy_config.local_ip = String::from("127.0.0.1");
        tcp_proxy_config.local_port = frpc_props.local_port;
        tcp_proxy_config.remote_port = frpc_props.remote_port;
        self.tcp_configs
            .insert("service".to_string(), tcp_proxy_config);

        Ok(())
    }
}
