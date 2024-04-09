use crate::state::State;
use firefly_device::*;

type C<'a> = wasmi::Caller<'a, State>;

pub(crate) fn read_left(mut caller: C) -> u32 {
    let state = caller.data_mut();
    // TODO: cache input in state
    let input = match state.device.read_input() {
        Some(InputState {
            left: Some(left), ..
        }) => left,
        _ => return 0,
    };
    let x = input.x as u16 as u32;
    let y = input.y as u16 as u32;
    x << 16 | y
}

pub(crate) fn read_buttons(mut _caller: C) -> u32 {
    todo!()
}
