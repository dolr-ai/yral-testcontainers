use testcontainers::{
    core::{ContainerPort, WaitFor},
    Image,
};

pub struct OffChainAgent {
    tag: String,
}

pub const OFF_CHAIN_AGENT_PORT: ContainerPort = ContainerPort::Tcp(50051);

impl OffChainAgent {
    pub fn new(tag: &str) -> Self {
        Self { tag: tag.into() }
    }
}

impl Image for OffChainAgent {
    fn name(&self) -> &str {
        "ghcr.io/yral-dapp/off-chain-agent"
    }

    fn tag(&self) -> &str {
        &self.tag
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::millis(1000)]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[OFF_CHAIN_AGENT_PORT]
    }
}
