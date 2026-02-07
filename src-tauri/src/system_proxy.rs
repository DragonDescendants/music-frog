use infiltrator_desktop::{SystemProxyState, proxy as core_proxy};

pub(crate) fn apply_system_proxy(endpoint: Option<&str>) -> anyhow::Result<()> {
    core_proxy::apply_system_proxy(endpoint)
}

pub(crate) fn read_system_proxy_state() -> anyhow::Result<SystemProxyState> {
    core_proxy::read_system_proxy_state()
}
