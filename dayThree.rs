use std::fs;
use std::collections::HashMap;
use std::vec::Vec;
use std::vec::IntoIter;

fn instruction_iter(path: &str) -> IntoIter<(i32, i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;

    let mut points = Vec::new();

    for instruction in path.split(",") {
        let mut chars = instruction.chars();
        let (direction, magnitude) = (
            chars.next().unwrap(),
            chars.as_str().parse::<i32>().unwrap()
        );

        for _i in 0..magnitude {
            match direction {
                'L' => x = x + 1,
                'R' => x = x - 1,
                'U' => y = y + 1,
                'D' => y = y - 1,
                _ => {}
            };
            steps = steps + 1;
            points.push((x, y, steps));
        }
    }
    
    points.into_iter()
}

fn main() {
    let input = fs::read_to_string("data/dayThree.txt")
        .expect("Unable to read input file");

    let mut first_wire_points = HashMap::new();
    
    let mut smallest_distance = i32::max_value();
    let mut fewest_steps = i32::max_value();

    for (wire, path) in input.lines().enumerate() {
        if wire == 0 {
            for (x, y, steps) in instruction_iter(path) {
                if !first_wire_points.contains_key(&(x, y)) {
                    first_wire_points.insert((x, y), steps);
                }
            }
        } else {
            for (x, y, steps) in instruction_iter(path) {
                if first_wire_points.contains_key(&(x, y)) {
                    let distance = x.abs() + y.abs();

                    if distance < smallest_distance {
                        smallest_distance = distance;
                    }

                    let total_steps = steps + *first_wire_points.get(&(x, y)).unwrap();

                    if total_steps < fewest_steps {
                        fewest_steps = total_steps;
                    }
                }
            }
        }
    }

    println!("Part one: {}", smallest_distance);
    println!("Part two: {}", fewest_steps);
}