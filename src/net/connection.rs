use super::*;
use firefly_device::*;

const MAX_PEERS: usize = 8;
const MSG_SIZE: usize = 64;
type Addr = <NetworkImpl as Network>::Addr;

pub(crate) struct Peer {
    /// If address is None, the peer is the current device.
    pub addr: Option<Addr>,
    pub name: heapless::String<16>,
}

/// Connection is a result of connector.
///
/// If you play games with friends, you establish the connection once
/// at the beginning of the evening and it stays on as long as
/// all the devices are turned on.
///
/// This object is allocated while your are in the launcher.
/// Its job is to launch an app for everyone when someone picks one to play.
pub(crate) struct Connection {
    pub(super) net: NetworkImpl,
    pub peers: heapless::Vec<Peer, MAX_PEERS>,
}

impl Connection {
    pub fn update(&mut self, device: &DeviceImpl) {
        let res = self.update_inner(device);
        if let Err(err) = res {
            device.log_error("netcode", err);
        }
    }
    fn update_inner(&mut self, device: &DeviceImpl) -> Result<(), NetcodeError> {
        if let Some((addr, msg)) = self.net.recv()? {
            self.handle_message(device, addr, msg)?;
        }
        Ok(())
    }

    fn handle_message(
        &mut self,
        device: &DeviceImpl,
        addr: Addr,
        raw: heapless::Vec<u8, MSG_SIZE>,
    ) -> Result<(), NetcodeError> {
        if raw.is_empty() {
            return Err(NetcodeError::EmptyBufferIn);
        }
        let msg = match Message::decode(&raw) {
            Ok(msg) => msg,
            Err(err) => return Err(NetcodeError::Deserialize(err)),
        };
        match msg {
            Message::Req(req) => self.handle_req(device, addr, req),
            Message::Resp(resp) => self.handle_resp(device, addr, resp),
        }
    }

    fn handle_req(
        &mut self,
        device: &DeviceImpl,
        addr: Addr,
        req: Req,
    ) -> Result<(), NetcodeError> {
        todo!()
    }

    fn handle_resp(
        &mut self,
        device: &DeviceImpl,
        addr: Addr,
        resp: Resp,
    ) -> Result<(), NetcodeError> {
        todo!()
    }
}
