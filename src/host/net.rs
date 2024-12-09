use crate::{
    error::HostError,
    net::FSPeer,
    state::{NetHandler, State},
};

use super::stats::get_friend;

type C<'a> = wasmi::Caller<'a, State>;

/// Get the index of the local peer.
pub(crate) fn get_me(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.get_me";
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

/// Get the map of peers that are currently online.
pub(crate) fn get_peers(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.get_peers";
    let handler = state.net_handler.get_mut();
    let NetHandler::FrameSyncer(syncer) = handler else {
        return 1;
    };
    let mut res = 0u32;
    for _peer in &syncer.peers {
        res = res << 1 | 1;
    }
    res
}

pub(crate) fn save_stash(mut caller: C, peer_id: u32, buf_ptr: u32, buf_len: u32) {
    let state = caller.data_mut();
    state.called = "net.save_stash";

    let Some(memory) = state.memory else {
        state.log_error(HostError::MemoryNotFound);
        return;
    };
    let (data, state) = memory.data_and_store_mut(&mut caller);
    let buf_ptr = buf_ptr as usize;
    let buf_len = buf_len as usize;
    let Some(buf) = data.get_mut(buf_ptr..(buf_ptr + buf_len)) else {
        state.log_error(HostError::OomPointer);
        return;
    };

    let mut handler = state.net_handler.replace(NetHandler::None);
    let peer = get_friend(&mut handler, peer_id);
    match peer {
        Some(peer) => save_stash_friend(state, peer, buf),
        None => {
            let buf = alloc::vec::Vec::from(buf);
            state.stash = Some(buf.into_boxed_slice());
            state.stash_dirty = true;
        }
    }
    state.net_handler.replace(handler);
}

fn save_stash_friend(state: &mut State, peer: &mut FSPeer, buf: &[u8]) {
    todo!()
}
