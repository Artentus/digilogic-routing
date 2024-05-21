#[derive(Debug)]
pub struct Segment<T> {
    pub start_inclusive: i32,
    pub end_inclusive: i32,
    pub value: T,
}

impl<T> Segment<T> {
    #[inline]
    fn len(&self) -> i32 {
        assert!(self.end_inclusive >= self.start_inclusive);
        self.end_inclusive - self.start_inclusive
    }
}

#[derive(Debug)]
pub struct SegmentTree<T> {
    segments: Vec<Segment<T>>,
    max_segment_len: i32,
}

impl<T> Default for SegmentTree<T> {
    #[inline]
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            max_segment_len: 0,
        }
    }
}

impl<T> SegmentTree<T> {
    pub fn build(&mut self, segments: impl IntoIterator<Item = Segment<T>>)
    where
        Segment<T>: Send,
    {
        use rayon::prelude::*;

        self.segments.clear();
        self.segments.extend(segments.into_iter());
        self.segments
            .par_sort_unstable_by_key(|segment| segment.start_inclusive);
        self.max_segment_len = self.segments.iter().map(Segment::len).max().unwrap_or(0);
    }

    fn find_start_index(&self, position: i32) -> usize {
        match self.segments.binary_search_by(|segment| {
            (segment.start_inclusive + self.max_segment_len).cmp(&position)
        }) {
            Ok(mut index) => loop {
                if index == 0 {
                    return 0;
                }

                if (self.segments[index].start_inclusive + self.max_segment_len) >= position {
                    index -= 1;
                } else {
                    return index + 1;
                }
            },
            Err(index) => index,
        }
    }

    fn find_end_index(&self, position: i32) -> usize {
        match self
            .segments
            .binary_search_by(|segment| segment.start_inclusive.cmp(&position))
        {
            Ok(mut index) => {
                while let Some(segment) = self.segments.get(index) {
                    if segment.start_inclusive <= position {
                        index += 1;
                    } else {
                        break;
                    }
                }

                index
            }
            Err(index) => index,
        }
    }

    pub fn iter_containing<'a>(&'a self, position: i32) -> impl Iterator<Item = &'a T> {
        let start_index = self.find_start_index(position);
        let end_index = self.find_end_index(position);

        #[cfg(debug_assertions)]
        {
            if let Some(start_segment) = self.segments.get(start_index) {
                assert!((start_segment.start_inclusive + self.max_segment_len) >= position);
            }

            if let Some(before_start_index) = start_index.checked_sub(1) {
                if let Some(before_start_segment) = self.segments.get(before_start_index) {
                    assert!(
                        (before_start_segment.start_inclusive + self.max_segment_len) < position
                    );
                    assert!(before_start_segment.end_inclusive < position);
                }
            }

            if let Some(end_segment) = self.segments.get(end_index) {
                assert!(end_segment.start_inclusive > position);
            }
        }

        self.segments[start_index..end_index]
            .iter()
            .filter(move |segment| segment.end_inclusive >= position)
            .inspect(move |segment| debug_assert!(segment.start_inclusive <= position))
            .map(|segment| &segment.value)
    }
}
