use itertools::Itertools;

fn input() -> Vec<usize> {
    include_str!("./inputs/2021/1.txt")
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()
        .expect("Couldn't parse input")
}

fn num_downward_steps(input: Vec<usize>) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(near, far)| far > near)
        .count()
}

fn num_downward_steps_grouped(input: Vec<usize>) -> usize {
    input
        .windows(3)
        .tuple_windows()
        .filter(|(near_group, far_group)| far_group.iter().sum::<usize>() > near_group.iter().sum())
        .count()
}

#[test]
fn part1() {
    assert_eq!(num_downward_steps(input()), 1316)
}

#[test]
fn part2() {
    assert_eq!(num_downward_steps_grouped(input()), 1344)
}
