use std::fs;

fn required_fuel(mass: i32) -> i32 {
  let fuel_for_mass = (mass / 3) - 2;
  if fuel_for_mass > 0 { fuel_for_mass + required_fuel(fuel_for_mass) } else { 0 }
}

fn main() {
  let input = fs::read_to_string("dayOne.txt")
    .expect("Unable to read file dayOne.txt");

  let mut total_fuel_so_far = 0;

  for weight in input.lines() {
    // It's likely the file will end in a new line
    if weight == "" {
      continue;
    }

    let mass: i32 = weight.parse().expect("Unable to parse mass!");
    total_fuel_so_far += required_fuel(mass);
  }

  println!("{}", total_fuel_so_far);
}