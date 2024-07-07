use crate::state::{NetHandler, State};

type C<'a> = wasmi::Caller<'a, State>;

/// Check if netplay is active.
pub(crate) fn is_online(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.is_online";
    let handler = state.net_handler.get_mut();
    let offline = matches!(handler, NetHandler::None);
    u32::from(!offline)
}

/// Get the index of the local player.
pub(crate) fn get_player_id(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.get_player_id";
    let handler = state.net_handler.get_mut();
    let NetHandler::FrameSyncer(syncer) = handler else {
        return 0;
    };
    for (peer, i) in syncer.peers.iter().zip(0u32..) {
        if peer.addr.is_none() {
            return i;
        }
    }
    unreachable!("list of peers has no local device")
}
