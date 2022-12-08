use std::ops::Not;

pub fn part_1(input: &str) -> usize {
    let window_size = 4;
    let input = input.chars().collect::<Vec<char>>();
    let option = input
        .windows(window_size)
        .enumerate()
        .find(|w| has_duplicate(w.1).not());
    option.unwrap().0 + window_size
}

pub fn part_2(input: &str) -> usize {
    let window_size = 14;
    let input = input.chars().collect::<Vec<char>>();
    let option = input
        .windows(window_size)
        .enumerate()
        .find(|w| has_duplicate(w.1).not());
    option.unwrap().0 + window_size
}

fn has_duplicate(input: &[char]) -> bool {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i] == input[j] {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        // given
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        // when
        let result = part_1(input);

        // then
        assert_eq!(result, 7);

        // given
        // given
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        // when
        let result = part_1(input);

        // then
        assert_eq!(result, 11);
    }

    #[test]
    fn part_2_test() {
        // given
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        // when
        let result = part_2(input);

        // then
        assert_eq!(result, 19);

        // given
        // given
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        // when
        let result = part_2(input);

        // then
        assert_eq!(result, 26);
    }

    #[test]
    fn has_duplicate_test() {
        // given
        let input = "abca".chars().collect::<Vec<_>>();

        // when
        let result = has_duplicate(&input);

        // then
        assert!(result);
    }
}
