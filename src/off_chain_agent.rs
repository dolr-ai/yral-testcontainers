use testcontainers::{
    core::{ContainerPort, WaitFor},
    Image,
};

pub struct OffChainAgent {
    tag: String,
}

impl OffChainAgent {
    pub fn new(tag: &str) -> Self {
        Sefl { tag: tag.into() }
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
}
