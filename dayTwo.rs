use std::fs;

fn run(mut memory: Vec<i32>) -> i32 {
  for i in (0..memory.len()).step_by(4) {
    let opcode = memory[i];

    match opcode {
      1 | 2 => {
        let op_a = memory[memory[i + 1] as usize];
        let op_b = memory[memory[i + 2] as usize];
        let dest = memory[i + 3] as usize;

        match opcode {
          1 => memory[dest] = op_a + op_b,
          2 => memory[dest] = op_a * op_b,
          _ => {}
        }
      }
      99 => { return memory[0] },
      _ => { panic!("Unknown opcode") },
    };
  }

  panic!("Reached end of memory without 99 instruction");
}

fn main() {
  let input = fs::read_to_string("data/dayTwo.txt")
    .expect("Unable to read dayTwo.txt");

  let input = input.split(",")
    .flat_map(str::parse::<i32>)
    .collect::<Vec<_>>();

  let mut memory = input.clone();

  memory[1] = 12;
  memory[2] = 2;

  println!("Part one: {}", run(memory));

  for noun in 0..=99 {
    for verb in 0..=99 {
      let mut memory = input.clone();

      memory[1] = noun;
      memory[2] = verb;

      if run(memory) == 19690720 {
        println!("Part two: {}", 100 * noun + verb);
        return;
      }
    }
  }

  println!("No solution found for part two");
}