use crate::state::State;

type C<'a> = wasmi::Caller<'a, State>;

/// Check if netplay is active.
pub(crate) fn is_online(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.is_online";
    caller.data().online as u32
}

/// Get the index of the local player.
pub(crate) fn get_player_id(mut caller: C) -> u32 {
    let state = caller.data_mut();
    state.called = "net.get_player_id";
    0
}
