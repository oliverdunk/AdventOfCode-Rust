use std::fs;
use std::vec::Vec;

fn digits(value: i32) -> std::vec::Vec::<i32> {
    let mut digits = Vec::new();

    for e in (1..7).rev() {
        digits.push(value % (10i32.pow(e)) / 10i32.pow(e - 1));
    }

    digits
}

fn is_ascending(value: i32) -> bool {
    let mut last = -1;

    for digit in digits(value) {
        if digit < last {
            return false;
        }

        last = digit;
    }

    true
}

fn has_repeat(value: i32, exclude_larger: bool) -> bool {
    let digits = digits(value);
    let mut iter = digits.iter().peekable();
    
    let mut current_group = -1;
    let mut count = 0;
    
    let mut digit_ref = iter.next();
    while digit_ref != None {
        let digit = *digit_ref.unwrap();

        if digit != current_group {
            if count >= 2 && !exclude_larger {
                return true;
            }

            if count == 2 {
                return true;
            }

            current_group = digit;
            count = 1;
        } else {
            count = count + 1
        }

        digit_ref = iter.next();
    }

    if exclude_larger {
        return count == 2;
    } else {
        return count >= 2;
    }
}

fn get_possible_passwords(start: i32, end: i32, exclude_larger: bool) -> usize {
    (start..end)
        .filter(|&x| is_ascending(x))
        .filter(|&x| has_repeat(x, exclude_larger))
        .count()
}

fn main() {
    let input = fs::read_to_string("data/dayFour.txt")
        .expect("Unable to read input file");
    let mut input = input.split("-")
        .flat_map(str::parse::<i32>);

    let (start, end) = (
        input.next().unwrap(),
        input.next().unwrap()
    );

    println!("Part One: {}", get_possible_passwords(start, end, false));
    println!("Part Two: {}", get_possible_passwords(start, end, true));
}