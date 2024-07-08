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

/// Get the index of the local peer.
pub(crate) fn get_peer_id(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.get_peer_id";
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

/// Get the number how many peers are currently online.
pub(crate) fn get_peer_count(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.get_peer_count";
    let handler = state.net_handler.get_mut();
    let NetHandler::FrameSyncer(syncer) = handler else {
        return 0;
    };
    syncer.peers.len() as u32
}

/// Get the map of peers that are currently online.
pub(crate) fn get_peers(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.get_peers";
    let handler = state.net_handler.get_mut();
    let NetHandler::FrameSyncer(syncer) = handler else {
        return 0;
    };
    let mut res = 0u32;
    for _peer in &syncer.peers {
        res = res << 1 | 1;
    }
    res
}
