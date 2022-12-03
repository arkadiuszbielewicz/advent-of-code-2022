pub fn part_1(input: &str) -> u32 {
    input.lines().map(calculate_one_line).sum()
}

fn calculate_one_line(input: &str) -> u32 {
    let (first_half, second_half) = split_half(input);
    let duplicates = find_duplicate(first_half, second_half).unwrap();
    return to_score(duplicates);
}

fn to_score(duplicates: char) -> u32 {
    if duplicates.is_uppercase() {
        duplicates as u32 - 38
    } else {
        duplicates as u32 - 96
    }
}

fn split_half(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

fn find_duplicate(first: &str, second: &str) -> Option<char> {
    first.chars().filter(|c| second.chars().find(|o| o == c).is_some()).next()
}

pub fn part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.chunks(3).map(|chunk| {
        let (r1, r2, r3) = (chunk[0], chunk[1], chunk[2]);
        find_in_group(r1, r2, r3).unwrap()
    })
        .map(to_score)
        .sum()
}

fn find_in_group(r1: &str, r2: &str, r3: &str) -> Option<char> {
    r1.chars().filter(|c|
        {
            r2.chars().find(|o| o == c).is_some() &&
                r3.chars().find(|o| o == c).is_some()
        }).next()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_in_half() {
        // given
        let s = "mScqSqqgcfPCqGPZcfGNSvTNsVVNSjNvWSNsNz";

        // when
        let (first_half, second_half) = split_half(s);

        // then
        assert_eq!(first_half, "mScqSqqgcfPCqGPZcfG");
        assert_eq!(second_half, "NSvTNsVVNSjNvWSNsNz");
    }

    #[test]
    fn test_find_duplicate() {
        // given
        let a = "abcd";
        let b = "efghd";

        // when
        let duplicate = find_duplicate(a, b);

        // then
        assert!(duplicate.is_some());
        assert_eq!(duplicate.unwrap(), 'd');
    }

    #[test]
    fn test_part_1() {
        // given
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        // when
        let result = part_1(input);

        // then
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part_2() {
        // given
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        // when
        let result = part_2(input);

        // then
        assert_eq!(result, 70);
    }
}