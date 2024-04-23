use crate::state::State;

type C<'a> = wasmi::Caller<'a, State>;

/// Check if netplay is active.
pub(crate) fn is_online(caller: C) -> u32 {
    caller.data().online as u32
}

/// Get the index of the local player.
pub(crate) fn get_player_id(_caller: C) -> u32 {
    0
}
