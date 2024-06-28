use crate::state::State;
use firefly_device::*;

type C<'a> = wasmi::Caller<'a, State>;

pub(crate) fn read_pad(mut caller: C, _player: u32) -> u32 {
    let state = caller.data_mut();
    state.called = "input.read_pad";
    let input = match &state.input {
        Some(InputState { pad: Some(pad), .. }) => pad,
        _ => return 0xffff,
    };
    let x = input.x as u16 as u32;
    let y = input.y as u16 as u32;
    x << 16 | y
}

pub(crate) fn read_buttons(mut caller: C, _player: u32) -> u32 {
    let state = caller.data_mut();
    state.called = "input.read_buttons";
    let Some(input) = &state.input else {
        return 0;
    };
    let mut res: u32 = 0;
    for button in input.buttons.into_iter().rev() {
        res = (res << 1) | u32::from(button);
    }
    res
}
