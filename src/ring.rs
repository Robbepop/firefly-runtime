use core::mem::MaybeUninit;

const BUF_SIZE: usize = 5;
const MAX_DRIFT: usize = BUF_SIZE / 2;

/// Circular buffer designed to keep a short history and a short look-ahead
/// for netowrk device state updates. The goal is to be able to reply recent frames
/// as well as not to loose frames received earlier than expected.
pub(crate) struct RingBuf<T> {
    frame: usize,
    data:  [MaybeUninit<T>; BUF_SIZE],
}

impl<T> RingBuf<T> {
    const INIT: MaybeUninit<T> = MaybeUninit::uninit();

    pub fn new() -> Self {
        Self {
            frame: 0,
            data:  [Self::INIT; BUF_SIZE],
        }
    }

    pub fn advance(&mut self) {
        self.frame += 1
    }

    pub fn insert(&mut self, frame: usize, val: T) {
        // Max drift ensures that too old or too ahead frame doesn't override
        // the frame that is closer to what we currently need.
        if self.frame.abs_diff(frame) > MAX_DRIFT {
            return;
        }
        let index = frame % BUF_SIZE;
        self.data[index] = MaybeUninit::new(val);
    }

    pub fn get(&mut self, frame: usize) -> Option<T> {
        if self.frame.abs_diff(frame) > MAX_DRIFT {
            return None;
        }
        let index = frame % BUF_SIZE;
        let val = self.data.get(index);
        val.map(|v| unsafe { v.assume_init_read() })
    }
}
