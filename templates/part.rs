/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect(
        "Input file exists and is readable in the root member's directory: day-XX/input.txt",
    );
    let answer = solve(&input);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_compute() {
        let tests = vec![
            ("", 0)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
