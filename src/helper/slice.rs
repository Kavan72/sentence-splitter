use std::ops::{Bound, RangeBounds};

fn range_to_begin_end(range: impl RangeBounds<usize>) -> (usize, usize) {
    let begin = match range.start_bound() {
        Bound::Included(&b) => b,
        Bound::Excluded(&b) => b + 1,
        Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        Bound::Included(&b) => b + 1,
        Bound::Excluded(&b) => b,
        Bound::Unbounded => usize::MAX,
    };

    (begin, end)
}

pub trait StringSlice {
    fn try_slice(&self, range: impl RangeBounds<usize>) -> Option<&str>;
    fn try_substring(&self, begin: usize, end: usize) -> Option<&str>;
}

impl StringSlice for str {

    fn try_slice(&self, range: impl RangeBounds<usize>) -> Option<&str> {
        let (begin, end) = range_to_begin_end(range);
        self.try_substring(begin, end)
    }

    fn try_substring(&self, begin: usize, end: usize) -> Option<&str> {
        if begin > end {
            None
        } else {
            let mut ch_idx = self.char_indices().map(|(i, _c)| i);

            let len = self.len();
            let begin_ch = ch_idx.nth(begin).unwrap_or(len);
            let end_ch = ch_idx.nth(end - begin - 1).unwrap_or(len);

            unsafe { Some(&self.get_unchecked(begin_ch..end_ch)) }
        }
    }
}
