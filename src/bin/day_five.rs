use std::fs;
use std::vec;

fn get_params(memory: &Vec<i32>, pc: usize, count: i32) -> Vec<i32> {
  let mut params = vec::Vec::new();
  let instruction = memory[pc];

  for i in 1..=count {
    let mut mode = 0;
    let param = memory[pc + i as usize];

    // Sometimes, the instruction doesn't bother to specify the parameter mode
    if instruction >= i32::pow(10, (i + 1) as u32) {
      mode = (instruction / i32::pow(10, (i + 1) as u32)) % 10;
    }

    match mode {
      0 => params.push(memory[param as usize]),
      1 => params.push(param),
      _ => panic!("Unknown parameter mode: {}", mode)
    }
  }

  params
}

fn run(mut memory: Vec<i32>, mut input: Vec<i32>) -> (i32, Vec<i32>) {
  let mut pc = 0;
  let mut output = vec::Vec::new();

  loop {
    let opcode = memory[pc] % 100;

    match opcode {
      1 | 2 => {
        let params = get_params(&memory, pc, 2);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = memory[pc + 3] as usize;
  
        match opcode {
          1 => memory[dest] = op_a + op_b,
          2 => memory[dest] = op_a * op_b,
          _ => {}
        }
  
        pc = pc + 4
      },
      3 => {
        let dest = memory[pc + 1] as usize;
        memory[dest] = input.pop().unwrap();
        pc = pc + 2;
      },
      4 => {
        output.push(*get_params(&memory, pc, 1).get(0).unwrap());
        pc = pc + 2;
      },
      5 => {
        let params = get_params(&memory, pc, 2);
        pc = if *params.get(0).unwrap() != 0 { *params.get(1).unwrap() as usize } else { pc + 3 }
      },
      6 => {
        let params = get_params(&memory, pc, 2);
        pc = if *params.get(0).unwrap() == 0 { *params.get(1).unwrap() as usize } else { pc + 3 }
      },
      7 => {
        let params = get_params(&memory, pc, 2);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = memory[pc + 3] as usize;
  
        memory[dest] = if op_a < op_b { 1 } else { 0 };
        pc = pc + 4;
      },
      8 => {
        let params = get_params(&memory, pc, 2);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = memory[pc + 3] as usize;
  
        memory[dest] = if op_a == op_b { 1 } else { 0 };
        pc = pc + 4;
      },
      99 => { return (memory[0], output) },
      _ => { panic!("Unknown opcode") },
    };
  }
}

fn main() {
  let input = fs::read_to_string("data/dayFive.txt")
    .expect("Unable to read dayFive.txt");

  let input = input.split(",")
    .flat_map(str::parse::<i32>)
    .collect::<Vec<_>>();

  let (exit_code, output) = run(input.clone(), vec![1]);
  println!("Part one (exit code {}): {:?}", exit_code, output);

  let (exit_code, output) = run(input.clone(), vec![5]);
  println!("Part two (exit code {}): {:?}", exit_code, output);
}