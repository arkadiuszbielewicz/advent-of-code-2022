use std::ops::{Not, Range};

pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(l, r)| (range_from_str(l).unwrap(), range_from_str(r).unwrap()))
        .map(|(l, r)| is_within(&l, &r))
        .filter(|r| *r)
        .count() as u64
}

pub fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(l, r)| (range_from_str(l).unwrap(), range_from_str(r).unwrap()))
        .map(|(l, r)| overlaps(&l, &r))
        .filter(|r| *r)
        .count() as u64
}

fn range_from_str(input: &str) -> Option<Range<u64>> {
    input
        .split_once("-")
        .and_then(|(start, end)| {
            str::parse::<u64>(start)
                .ok()
                .zip(str::parse::<u64>(end).ok())
        })
        .map(|(start, end)| start..end)
}

fn is_within<Idx: PartialOrd<Idx>>(first: &Range<Idx>, second: &Range<Idx>) -> bool {
    first.start <= second.start && first.end >= second.end
        || second.start <= first.start && second.end >= first.end
}

fn overlaps<Idx: PartialOrd<Idx>>(first: &Range<Idx>, second: &Range<Idx>) -> bool {
    (first.end < second.start || second.end < first.start).not()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_within_test() {
        // given
        let first = 0..5;
        let second = 3..4;

        // when
        let within = is_within(&first, &second);

        // then
        assert!(within);

        // when
        let within = is_within(&second, &first);

        // then
        assert!(within);
    }

    #[test]
    fn part_1_test() {
        // given
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        // when
        let result = part_1(input);

        // then
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_test() {
        // given
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        // when
        let result = part_2(input);

        // then
        assert_eq!(result, 4);
    }
}
