use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum Proxy {
    // Standard Protocols
    Shadowsocks(Shadowsocks),
    Vmess(Vmess),
    Trojan(Trojan),
    Hysteria2(Hysteria2),
    WireGuard(WireGuard),
    Tuic(Tuic),
    Vless(Vless),
    Http(Http),
    Socks5(Socks5),
    Snell(Snell),
    Direct(Direct),
    Reject(Reject),
    
    // Group Types
    Selector(ProxyGroup),
    URLTest(ProxyGroup),
    Fallback(ProxyGroup),
    LoadBalance(ProxyGroup),
    
    // Catch-all for unknown types to ensure forward compatibility
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProxyBase {
    pub name: String,
    pub udp: bool,
    pub history: Vec<ProxyHistory>,
    pub alive: bool,
    pub delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProxyHistory {
    pub time: String,
    pub delay: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Shadowsocks {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
    pub cipher: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Vmess {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
    pub uuid: String,
    #[serde(rename = "alterId")]
    pub alter_id: u32,
    pub cipher: String,
    pub tls: bool,
    pub network: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Trojan {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
    pub sni: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Hysteria2 {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
    pub auth: Option<String>,
    pub sni: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WireGuard {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
    pub ip: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tuic {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Vless {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Http {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Socks5 {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Snell {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub server: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Direct {
    #[serde(flatten)]
    pub base: ProxyBase,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Reject {
    #[serde(flatten)]
    pub base: ProxyBase,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProxyGroup {
    pub name: String,
    pub now: String,
    pub all: Vec<String>,
    pub history: Vec<ProxyHistory>,
}

impl Proxy {
    pub fn name(&self) -> &str {
        match self {
            Proxy::Shadowsocks(p) => &p.base.name,
            Proxy::Vmess(p) => &p.base.name,
            Proxy::Trojan(p) => &p.base.name,
            Proxy::Hysteria2(p) => &p.base.name,
            Proxy::WireGuard(p) => &p.base.name,
            Proxy::Tuic(p) => &p.base.name,
            Proxy::Vless(p) => &p.base.name,
            Proxy::Http(p) => &p.base.name,
            Proxy::Socks5(p) => &p.base.name,
            Proxy::Snell(p) => &p.base.name,
            Proxy::Direct(p) => &p.base.name,
            Proxy::Reject(p) => &p.base.name,
            Proxy::Selector(p) | Proxy::URLTest(p) | Proxy::Fallback(p) | Proxy::LoadBalance(p) => {
                &p.name
            }
            Proxy::Unknown => "",
        }
    }

    pub fn proxy_type(&self) -> &str {
        match self {
            Proxy::Shadowsocks(_) => "Shadowsocks",
            Proxy::Vmess(_) => "Vmess",
            Proxy::Trojan(_) => "Trojan",
            Proxy::Hysteria2(_) => "Hysteria2",
            Proxy::WireGuard(_) => "WireGuard",
            Proxy::Tuic(_) => "Tuic",
            Proxy::Vless(_) => "Vless",
            Proxy::Http(_) => "Http",
            Proxy::Socks5(_) => "Socks5",
            Proxy::Snell(_) => "Snell",
            Proxy::Direct(_) => "Direct",
            Proxy::Reject(_) => "Reject",
            Proxy::Selector(_) => "Selector",
            Proxy::URLTest(_) => "URLTest",
            Proxy::Fallback(_) => "Fallback",
            Proxy::LoadBalance(_) => "LoadBalance",
            Proxy::Unknown => "Unknown",
        }
    }

    pub fn udp(&self) -> bool {
        match self {
            Proxy::Shadowsocks(p) => p.base.udp,
            Proxy::Vmess(p) => p.base.udp,
            Proxy::Trojan(p) => p.base.udp,
            Proxy::Hysteria2(p) => p.base.udp,
            Proxy::WireGuard(p) => p.base.udp,
            Proxy::Tuic(p) => p.base.udp,
            Proxy::Vless(p) => p.base.udp,
            Proxy::Http(p) => p.base.udp,
            Proxy::Socks5(p) => p.base.udp,
            Proxy::Snell(p) => p.base.udp,
            Proxy::Direct(p) => p.base.udp,
            Proxy::Reject(p) => p.base.udp,
            _ => false,
        }
    }

    pub fn history(&self) -> &[ProxyHistory] {
        match self {
            Proxy::Shadowsocks(p) => &p.base.history,
            Proxy::Vmess(p) => &p.base.history,
            Proxy::Trojan(p) => &p.base.history,
            Proxy::Hysteria2(p) => &p.base.history,
            Proxy::WireGuard(p) => &p.base.history,
            Proxy::Tuic(p) => &p.base.history,
            Proxy::Vless(p) => &p.base.history,
            Proxy::Http(p) => &p.base.history,
            Proxy::Socks5(p) => &p.base.history,
            Proxy::Snell(p) => &p.base.history,
            Proxy::Direct(p) => &p.base.history,
            Proxy::Reject(p) => &p.base.history,
            Proxy::Selector(p) | Proxy::URLTest(p) | Proxy::Fallback(p) | Proxy::LoadBalance(p) => {
                &p.history
            }
            Proxy::Unknown => &[],
        }
    }

    pub fn all(&self) -> Option<&[String]> {
        match self {
            Proxy::Selector(p) | Proxy::URLTest(p) | Proxy::Fallback(p) | Proxy::LoadBalance(p) => {
                Some(&p.all)
            }
            _ => None,
        }
    }

    pub fn now(&self) -> Option<&str> {
        match self {
            Proxy::Selector(p) | Proxy::URLTest(p) | Proxy::Fallback(p) | Proxy::LoadBalance(p) => {
                Some(&p.now)
            }
            _ => None,
        }
    }

    pub fn is_group(&self) -> bool {
        matches!(
            self,
            Proxy::Selector(_) | Proxy::URLTest(_) | Proxy::Fallback(_) | Proxy::LoadBalance(_)
        )
    }

    pub fn alive(&self) -> bool {
        match self {
            Proxy::Shadowsocks(p) => p.base.alive,
            Proxy::Vmess(p) => p.base.alive,
            Proxy::Trojan(p) => p.base.alive,
            Proxy::Hysteria2(p) => p.base.alive,
            Proxy::WireGuard(p) => p.base.alive,
            Proxy::Tuic(p) => p.base.alive,
            Proxy::Vless(p) => p.base.alive,
            Proxy::Http(p) => p.base.alive,
            Proxy::Socks5(p) => p.base.alive,
            Proxy::Snell(p) => p.base.alive,
            Proxy::Direct(p) => p.base.alive,
            Proxy::Reject(p) => p.base.alive,
            _ => true,
        }
    }

    pub fn delay(&self) -> Option<u32> {
        match self {
            Proxy::Shadowsocks(p) => p.base.delay,
            Proxy::Vmess(p) => p.base.delay,
            Proxy::Trojan(p) => p.base.delay,
            Proxy::Hysteria2(p) => p.base.delay,
            Proxy::WireGuard(p) => p.base.delay,
            Proxy::Tuic(p) => p.base.delay,
            Proxy::Vless(p) => p.base.delay,
            Proxy::Http(p) => p.base.delay,
            Proxy::Socks5(p) => p.base.delay,
            Proxy::Snell(p) => p.base.delay,
            Proxy::Direct(p) => p.base.delay,
            Proxy::Reject(p) => p.base.delay,
            _ => None,
        }
    }
}

pub type Proxies = HashMap<String, Proxy>;

