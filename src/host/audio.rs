use crate::error::HostError;
use crate::state::State;
use alloc::boxed::Box;
use firefly_audio::*;

type C<'a> = wasmi::Caller<'a, State>;

/// Add sine wave generator as a child for the given node.
pub(crate) fn add_sine(mut caller: C, parent_id: u32, freq: f32, phase: f32) -> u32 {
    let state = caller.data_mut();
    state.called = "audio.add_sine";
    let proc = Sine::new(freq, phase);
    add_node(state, parent_id, Box::new(proc))
}

/// Add square wave generator as a child for the given node.
pub(crate) fn add_square(mut caller: C, parent_id: u32, freq: f32, phase: f32) -> u32 {
    let state = caller.data_mut();
    state.called = "audio.add_square";
    let proc = Square::new(freq, phase);
    add_node(state, parent_id, Box::new(proc))
}

fn add_node(state: &mut State, parent_id: u32, proc: Box<dyn firefly_audio::Processor>) -> u32 {
    match state.audio.add_node(parent_id, proc) {
        Ok(id) => id,
        Err(err) => {
            state.log_error(HostError::AudioNode(err));
            0
        }
    }
}

/// Reset the given node.
pub(crate) fn reset(mut caller: C, node_id: u32) {
    let state = caller.data_mut();
    state.called = "audio.reset";
    match state.audio.get_node(node_id) {
        Ok(node) => node.reset(),
        Err(err) => state.log_error(HostError::AudioNode(err)),
    };
}

/// Reset the given node and all its child nodes.
pub(crate) fn reset_all(mut caller: C, node_id: u32) {
    let state = caller.data_mut();
    state.called = "audio.reset_all";
    match state.audio.get_node(node_id) {
        Ok(node) => node.reset_all(),
        Err(err) => state.log_error(HostError::AudioNode(err)),
    };
}

/// Remove all children from the node.
pub(crate) fn clear(mut caller: C, node_id: u32) {
    let state = caller.data_mut();
    state.called = "audio.clear";
    let res = state.audio.clear(node_id);
    if let Err(err) = res {
        state.log_error(HostError::AudioNode(err));
    }
}
