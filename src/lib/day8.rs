use grid::Grid;

pub fn part_1(input: &str) -> usize {
    let (width, grid) = input_to_grid(input);
    grid.iter()
        .enumerate()
        .map(|(idx, v)| (idx_to_crd(idx, width), v))
        .filter(|((x, y), v)| is_visible(*x, *y, **v, &grid))
        .count()
}

fn input_to_grid(input: &str) -> (usize, Grid<u32>) {
    let (input, width) = input
        .lines()
        .map(|l| {
            (
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>(),
                l.chars().count(),
            )
        })
        .fold((Vec::new(), 0), |(mut acc, _), (mut line, len)| {
            acc.append(&mut line);
            (acc, len)
        });
    let grid = Grid::from_vec(input, width);
    (width, grid)
}

pub fn part_2(input: &str) -> u32 {
    let (width, grid) = input_to_grid(input);
    grid.iter()
        .enumerate()
        .map(|(idx, v)| (idx_to_crd(idx, width), v))
        .map(|((x, y), v)| count_scenic_score(x, y, *v, &grid))
        .max()
        .unwrap()
}

fn idx_to_crd(idx: usize, width: usize) -> (usize, usize) {
    (idx / width, idx % width)
}

fn is_visible(x: usize, y: usize, v: u32, grid: &Grid<u32>) -> bool {
    if x == 0 || x >= grid.rows() - 1 || y == 0 || y >= grid.cols() - 1 {
        return true;
    }

    let mut is_visible = true;
    for i in (0..y).rev() {
        if grid[x][i] >= v {
            is_visible = false;
            break;
        }
    }
    if is_visible {
        return true;
    }

    let mut is_visible = true;
    for i in y + 1..grid.cols() {
        if grid[x][i] >= v {
            is_visible = false;
            break;
        }
    }
    if is_visible {
        return true;
    }

    let mut is_visible = true;
    for i in (0..x).rev() {
        if grid[i][y] >= v {
            is_visible = false;
            break;
        }
    }
    if is_visible {
        return true;
    }

    let mut is_visible = true;
    for i in x + 1..grid.rows() {
        if grid[i][y] >= v {
            is_visible = false;
            break;
        }
    }
    is_visible
}

fn count_scenic_score(x: usize, y: usize, v: u32, grid: &Grid<u32>) -> u32 {
    let mut left_score = 0;
    for i in (0..y).rev() {
        left_score += 1;
        if grid[x][i] >= v {
            break;
        }
    }

    let mut right_score = 0;
    for i in y + 1..grid.cols() {
        right_score += 1;
        if grid[x][i] >= v {
            break;
        }
    }

    let mut top_score = 0;
    for i in (0..x).rev() {
        top_score += 1;
        if grid[i][y] >= v {
            break;
        }
    }

    let mut bottom_score = 0;
    for i in x + 1..grid.rows() {
        bottom_score += 1;
        if grid[i][y] >= v {
            break;
        }
    }
    left_score * right_score * top_score * bottom_score
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        // given
        let input = "30373
25512
65332
33549
35390";

        // when
        let result = part_1(input);

        // then
        assert_eq!(result, 21);
    }

    #[test]
    fn part_2_test() {
        // given
        let input = "30373
25512
65332
33549
35390";

        // when
        let result = part_2(input);

        // then
        assert_eq!(result, 8);
    }
}
