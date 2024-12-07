use embedded_io::Write;
use firefly_hal::Device;

use crate::{
    error::HostError,
    state::{NetHandler, State},
};

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
    save_stash_to_fs(caller, buf_ptr, buf_len);
}

fn save_stash_to_fs(mut caller: C, buf_ptr: u32, buf_len: u32) {
    let state = caller.data_mut();
    let Some(memory) = state.memory else {
        state.log_error(HostError::MemoryNotFound);
        return;
    };
    let (data, state) = memory.data_and_store_mut(&mut caller);

    let path = &["data", state.id.author(), state.id.app(), "stash"];
    let mut file = match state.device.create_file(path) {
        Ok(file) => file,
        Err(err) => {
            state.log_error(err);
            return;
        }
    };
    let buf_ptr = buf_ptr as usize;
    let buf_len = buf_len as usize;
    let Some(buf) = data.get_mut(buf_ptr..(buf_ptr + buf_len)) else {
        state.log_error(HostError::OomPointer);
        return;
    };
    let Ok(file_size) = file.write(buf) else {
        state.log_error(HostError::FileWrite);
        return;
    };
    if file.flush().is_err() {
        state.log_error(HostError::FileFlush);
        return;
    }
    if file_size != buf_len {
        state.log_error(HostError::BufferSize);
    }
}
