pub fn part_1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|s| str::parse::<i64>(s).unwrap())
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> i64 {
    let mut sums = input
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|s| str::parse::<i64>(s).unwrap())
                .sum::<i64>()
        })
        .collect::<Vec<i64>>();
    sums.sort_by(|a, b| b.cmp(a));
    sums.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        // given
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        // when
        let result = part_1(&input);

        // then
        assert_eq!(result, 24000);
    }

    #[test]
    fn part_2_test() {
        // given
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        // when
        let result = part_2(&input);

        // then
        assert_eq!(result, 45000);
    }
}
