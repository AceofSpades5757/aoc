# Description

CLI to help with Advent of Code completions.

```sh
# Automatically Download Input, based on the day
aoc input

# Submit Answer, based on the day
aoc submit --part {number} # This will run the part, capture the output, and submit
echo 300 | aoc submit
echo 300 | aoc submit -
echo 300 | aoc submit --stdin

# Create new day
aoc new day
# Copy part_1 bin to part_2
aoc new part

# Test Code, based on the day
aoc test
# Run Code, based on the day
aoc run
```

# Config

`config.toml`

```toml
[formats]
# Each should be an integer
repo="advent-of-code-{year}"
day="day-{day}"
part="part-{part}.rs"

# TODO: This could be used for automatically submitting.
[commands]
run="cargo run --bin {file}"
test="cargo test"
```
