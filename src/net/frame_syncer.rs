use super::ring::RingBuf;
use super::*;
use firefly_device::*;

const REPEAT_EVERY: Duration = Duration::from_ms(5);
const MAX_PEERS: usize = 8;
const MSG_SIZE: usize = 64;
type Addr = <NetworkImpl as Network>::Addr;

pub(crate) struct FSPeer {
    /// If address is None, the peer is the current device.
    pub addr: Option<Addr>,
    pub name: heapless::String<16>,
    pub states: RingBuf<FrameState>,
}

pub(crate) struct FrameSyncer {
    pub frame: u32,
    pub peers: heapless::Vec<FSPeer, MAX_PEERS>,
}

impl FrameSyncer {
    pub fn ready(&self) -> bool {
        for peer in &self.peers {
            let state = peer.states.get(self.frame);
            if state.is_none() {
                return false;
            }
        }
        true
    }
}
