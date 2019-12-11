// Credit to https://www.reddit.com/r/adventofcode/comments/e8aw9j/2019_day_9_part_1_how_to_fix_203_error/,
// which helped me to finally understand destination addressing.

use std::fs;
use std::vec;

fn get_params(memory: &Vec<i64>, pc: usize, relative_base: isize, count: i64, has_dest: bool) -> Vec<i64> {
  let mut params = vec::Vec::new();
  let instruction = memory[pc];

  for i in 1..=count {
    let mut mode = 0;
    let param = memory[pc + i as usize];

    // Sometimes, the instruction doesn't bother to specify the parameter mode
    if instruction >= i64::pow(10, (i + 1) as u32) {
      mode = (instruction / i64::pow(10, (i + 1) as u32)) % 10;
    }

    if i != count || !has_dest {
      // This is an argument
      match mode {
        0 => params.push(memory[param as usize]),
        1 => params.push(param),
        2 => params.push(memory[(relative_base + param as isize) as usize]),
        _ => panic!("Unknown parameter mode: {}", mode)
      }
    } else {
      // This is a destination
      match mode {
        0 => params.push(param),
        1 => panic!("Parameter mode 1 not supported for dest"),
        2 => params.push((relative_base + param as isize) as i64),
        _ => panic!("Unknown parameter mode: {}", mode)
      }
    }
  }

  params
}

fn run_to_next_output(pc: &mut usize, relative_base: &mut isize, memory: &mut Vec<i64>, mut input: Vec<i64>) -> Option<i64> {
  // We want to use pop, but that works in the wrong order
  input.reverse();

  loop {
    let opcode = memory[*pc] % 100;

    match opcode {
      1 | 2 => {
        let params = get_params(&memory, *pc, *relative_base, 3, true);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = *params.get(2).unwrap() as usize;
  
        match opcode {
          1 => memory[dest] = op_a + op_b,
          2 => memory[dest] = op_a * op_b,
          _ => {}
        }
  
        *pc = *pc + 4
      },
      3 => {
        let params = get_params(&memory, *pc, *relative_base, 1, true);
        memory[*params.get(0).unwrap() as usize] = input.pop().unwrap();
        *pc = *pc + 2;
      },
      4 => {
        let to_output = *get_params(&memory, *pc, *relative_base, 1, false).get(0).unwrap();
        *pc = *pc + 2;
        return Some(to_output);
      },
      5 => {
        let params = get_params(&memory, *pc, *relative_base, 2, false);
        *pc = if *params.get(0).unwrap() != 0 { *params.get(1).unwrap() as usize } else { *pc + 3 }
      },
      6 => {
        let params = get_params(&memory, *pc, *relative_base, 2, false);
        *pc = if *params.get(0).unwrap() == 0 { *params.get(1).unwrap() as usize } else { *pc + 3 }
      },
      7 => {
        let params = get_params(&memory, *pc, *relative_base, 3, true);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = *params.get(2).unwrap() as usize;
  
        memory[dest] = if op_a < op_b { 1 } else { 0 };
        *pc = *pc + 4;
      },
      8 => {
        let params = get_params(&memory, *pc, *relative_base, 3, true);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = *params.get(2).unwrap() as usize;
  
        memory[dest] = if op_a == op_b { 1 } else { 0 };
        *pc = *pc + 4;
      },
      9 => {
        let params = get_params(&memory, *pc, *relative_base, 1, false);
        *relative_base = *relative_base + (*params.get(0).unwrap() as isize);
        *pc = *pc + 2;
      },
      99 => { return None; },
      _ => { panic!("Unknown opcode") },
    };
  }
}

fn main() {
  let input = fs::read_to_string("data/dayNine.txt")
    .expect("Unable to read dayNine.txt");

  let mut input = input.split(",")
    .flat_map(str::parse::<i64>)
    .collect::<Vec<_>>();

  // We need more memory than the program uses
  input.extend(vec![0; 1000000]);

  let result = run_to_next_output(&mut 0, &mut 0, &mut input, vec![1]);
  println!("Part one: {}", result.unwrap());

  let result = run_to_next_output(&mut 0, &mut 0, &mut input, vec![2]);
  println!("Part two: {}", result.unwrap());
}