#[cfg(test)]
mod tests {
    use crate::proxy::Proxy;
    use crate::proxy::types::{Shadowsocks, Vmess, Hysteria2};

    #[test]
    fn test_deserialize_all_proxy_types() {
        let json = r#"{
            "proxies": {
                "SS-Node": {
                    "type": "Shadowsocks",
                    "name": "SS-Node",
                    "udp": true,
                    "history": [{"time": "2024", "delay": 50}],
                    "alive": true,
                    "server": "1.1.1.1",
                    "port": 8388,
                    "cipher": "aes-256-gcm"
                },
                "Vmess-Node": {
                    "type": "Vmess",
                    "name": "Vmess-Node",
                    "udp": false,
                    "history": [],
                    "alive": true,
                    "server": "2.2.2.2",
                    "port": 443,
                    "uuid": "uuid-123",
                    "alter_id": 0,
                    "alterId": 0,
                    "cipher": "auto",
                    "tls": true,
                    "network": "ws"
                },
                "Hy2-Node": {
                    "type": "Hysteria2",
                    "name": "Hy2-Node",
                    "udp": true,
                    "history": [],
                    "alive": true,
                    "server": "3.3.3.3",
                    "port": 443,
                    "auth": "pass",
                    "sni": "example.com"
                },
                "Global-Group": {
                    "type": "Selector",
                    "name": "Global-Group",
                    "now": "SS-Node",
                    "all": ["SS-Node", "Vmess-Node"],
                    "history": []
                }
            }
        }"#;

        let resp: crate::types::ProxiesResponse = serde_json::from_str(json).unwrap();
        let proxies = resp.proxies;

        // Check Shadowsocks
        if let Proxy::Shadowsocks(ss) = proxies.get("SS-Node").unwrap() {
            assert_eq!(ss.base.name, "SS-Node");
            assert_eq!(ss.server, "1.1.1.1");
            assert_eq!(ss.cipher, "aes-256-gcm");
        } else {
            panic!("Expected Shadowsocks");
        }

        // Check Vmess
        if let Proxy::Vmess(vm) = proxies.get("Vmess-Node").unwrap() {
            assert_eq!(vm.uuid, "uuid-123");
            assert!(vm.tls);
        } else {
            panic!("Expected Vmess");
        }

        // Check Hysteria2
        if let Proxy::Hysteria2(hy) = proxies.get("Hy2-Node").unwrap() {
            assert_eq!(hy.sni, Some("example.com".to_string()));
        } else {
            panic!("Expected Hysteria2");
        }

        // Check Group
        if let Proxy::Selector(g) = proxies.get("Global-Group").unwrap() {
            assert_eq!(g.now, "SS-Node");
            assert_eq!(g.all.len(), 2);
        } else {
            panic!("Expected Selector group");
        }
    }

    #[test]
    fn test_deserialize_unknown_type() {
        let json = r#"{
            "proxies": {
                "Future-Node": {
                    "type": "NewProtocolX",
                    "name": "Future-Node",
                    "udp": true
                }
            }
        }"#;
        let resp: crate::types::ProxiesResponse = serde_json::from_str(json).unwrap();
        assert!(matches!(resp.proxies.get("Future-Node").unwrap(), Proxy::Unknown));
    }
}
