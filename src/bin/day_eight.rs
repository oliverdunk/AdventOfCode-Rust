use std::cmp::Ordering;
use std::fs;

fn get_layers(input: String, width: i32, height: i32) -> Vec<String> {
    input.as_bytes()
        .chunks((width * height) as usize)
        .map(|chunk| std::str::from_utf8(chunk).unwrap().to_string())
        .collect()
}

fn count_occurrences(input: &String, of: char) -> i32 {
    (&input.len() - input.replace(of, "").len()) as i32
}

fn get_ordering(order: i32) -> Ordering {
    if order == 0 {
        return Ordering::Equal;
    }

    if order > 0 { Ordering::Greater } else { Ordering::Less }
}

fn main() {
    let input = fs::read_to_string("data/dayEight.txt")
        .expect("Unable to read dayEight.txt");

    let width = 25;
    let height = 6;

    let layers = get_layers(input, width, height);

    let fewest_zeros = layers.iter()
        .min_by(|a, b| get_ordering(count_occurrences(*a, '0') - count_occurrences(*b, '0')))
        .unwrap();

    let ones = count_occurrences(fewest_zeros, '1');
    let twos = count_occurrences(fewest_zeros, '2');

    println!("Part one: {}", ones * twos);

    println!("Part two:");

    let pixels = width * height;
    let mut image = vec!["0"; pixels as usize];

    for pixel in 0..pixels {
        for layer in layers.iter().rev() {
            match layer.chars().nth(pixel as usize).unwrap() {
                '0' => image[pixel as usize] = " ",
                '1' => image[pixel as usize] = "•",
                _ => {},
            }
        }
    }

    for row in image.chunks(width as usize) {
        println!("{}", row.join("  "));
    }
}