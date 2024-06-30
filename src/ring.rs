use core::mem::MaybeUninit;

const BUF_SIZE: usize = 5;
const MAX_DRIFT: usize = BUF_SIZE / 2;

/// Circular buffer designed to keep a short history and a short look-ahead
/// for netowrk device state updates. The goal is to be able to reply recent frames
/// as well as not to loose frames received earlier than expected.
#[derive(Debug)]
pub(crate) struct RingBuf<T: Copy> {
    frame: usize,
    data:  [Option<(usize, T)>; BUF_SIZE],
}

impl<T: Copy> RingBuf<T> {
    const INIT: Option<(usize, T)> = None;

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
        self.data[index] = Some((frame, val));
    }

    pub fn get(&mut self, frame: usize) -> Option<T> {
        if self.frame.abs_diff(frame) > MAX_DRIFT {
            return None;
        }
        let index = frame % BUF_SIZE;
        let val = self.data.get(index)?;
        let (act_frame, val) = (*val)?;
        if act_frame != frame {
            return None;
        }
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buf() {
        let mut b: RingBuf<i32> = RingBuf::new();
        for i in 0..20 {
            assert_eq!(b.get(i), None);
        }
        for i in 0..20 {
            b.insert(i, 60 + i as i32);
        }
        // only the current frame (0) and 2 frames ahead must be inserted
        assert_eq!(b.get(0), Some(60));
        assert_eq!(b.get(1), Some(61));
        assert_eq!(b.get(2), Some(62));
        assert_eq!(b.get(3), None);

        // advance 10 frames forward
        for _ in 0..10 {
            b.advance();
        }
        assert_eq!(b.frame, 10);
        // all existing old frames must be ignored
        for i in 0..20 {
            assert_eq!(b.get(i), None);
        }
        // insert lots of frames, only the current frame, 2 before, and 2 after
        // must be inserted.
        for i in 0..20 {
            b.insert(i, 60 + i as i32);
        }
        for i in 0..=7 {
            assert_eq!(b.get(i), None);
        }
        assert_eq!(b.get(8), Some(68));
        assert_eq!(b.get(9), Some(69));
        assert_eq!(b.get(10), Some(70));
        assert_eq!(b.get(11), Some(71));
        assert_eq!(b.get(12), Some(72));
        for i in 13..=20 {
            assert_eq!(b.get(i), None);
        }
    }
}
