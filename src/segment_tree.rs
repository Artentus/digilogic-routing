#[derive(Debug)]
pub struct Segment<R, T> {
    pub start_inclusive: R,
    pub end_inclusive: R,
    pub value: T,
}

impl<R: PartialEq, T> PartialEq for Segment<R, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.start_inclusive.eq(&other.start_inclusive)
    }
}

impl<R: Eq, T> Eq for Segment<R, T> {}

impl<R: PartialOrd, T> PartialOrd for Segment<R, T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start_inclusive.partial_cmp(&other.start_inclusive)
    }
}

impl<R: Ord, T> Ord for Segment<R, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_inclusive.cmp(&other.start_inclusive)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct SegmentTree<R, T> {
    segments: Vec<Segment<R, T>>,
}

impl<R, T> Default for SegmentTree<R, T> {
    #[inline]
    fn default() -> Self {
        Self {
            segments: Vec::new(),
        }
    }
}

impl<R: Ord, T> SegmentTree<R, T> {
    pub fn build(&mut self, segments: impl IntoIterator<Item = Segment<R, T>>)
    where
        Segment<R, T>: Send,
    {
        use rayon::prelude::*;

        self.segments.clear();
        self.segments.extend(
            segments
                .into_iter()
                .inspect(|segment| debug_assert!(segment.start_inclusive <= segment.end_inclusive)),
        );
        self.segments.par_sort_unstable();
    }

    pub fn iter_containing<'a>(&'a self, position: &'a R) -> impl Iterator<Item = &'a T> {
        let mut start_index = match self
            .segments
            .binary_search_by(|segment| segment.start_inclusive.cmp(position))
        {
            Ok(index) | Err(index) => index,
        };

        loop {
            let Some(segment) = self.segments.get(start_index) else {
                break;
            };

            if segment.start_inclusive > *position {
                start_index += 1;
                break;
            }

            match start_index.checked_sub(1) {
                Some(index) => start_index = index,
                None => break,
            }
        }

        self.segments[start_index..]
            .iter()
            .take_while(|segment| segment.start_inclusive <= *position)
            .filter(|segment| segment.end_inclusive >= *position)
            .map(|segment| &segment.value)
    }
}
