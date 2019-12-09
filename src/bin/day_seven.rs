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

fn run_to_next_output(pc: &mut usize, memory: &mut Vec<i32>, mut input: Vec<i32>) -> Option<i32> {
  // We want to use pop, but that works in the wrong order
  input.reverse();

  loop {
    let opcode = memory[*pc] % 100;

    match opcode {
      1 | 2 => {
        let params = get_params(&memory, *pc, 2);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = memory[*pc + 3] as usize;
  
        match opcode {
          1 => memory[dest] = op_a + op_b,
          2 => memory[dest] = op_a * op_b,
          _ => {}
        }
  
        *pc = *pc + 4
      },
      3 => {
        let dest = memory[*pc + 1] as usize;
        memory[dest] = input.pop().unwrap();
        *pc = *pc + 2;
      },
      4 => {
        let to_output = *get_params(&memory, *pc, 1).get(0).unwrap();
        *pc = *pc + 2;
        return Some(to_output);
      },
      5 => {
        let params = get_params(&memory, *pc, 2);
        *pc = if *params.get(0).unwrap() != 0 { *params.get(1).unwrap() as usize } else { *pc + 3 }
      },
      6 => {
        let params = get_params(&memory, *pc, 2);
        *pc = if *params.get(0).unwrap() == 0 { *params.get(1).unwrap() as usize } else { *pc + 3 }
      },
      7 => {
        let params = get_params(&memory, *pc, 2);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = memory[*pc + 3] as usize;
  
        memory[dest] = if op_a < op_b { 1 } else { 0 };
        *pc = *pc + 4;
      },
      8 => {
        let params = get_params(&memory, *pc, 2);

        let op_a = *params.get(0).unwrap();
        let op_b = *params.get(1).unwrap();
        let dest = memory[*pc + 3] as usize;
  
        memory[dest] = if op_a == op_b { 1 } else { 0 };
        *pc = *pc + 4;
      },
      99 => { return None; },
      _ => { panic!("Unknown opcode") },
    };
  }
}

fn run_amplifiers(program: Vec<i32>, phases: Vec<i32>) -> i32 {
  // For each amplifier, we store the memory and pc
  let mut amplifier_states: Vec<(Vec<i32>, usize)> = Vec::new();

  let mut last_output = 0;

  loop {
    for amplifier in 0..phases.len() {
      // Create state on first iteration
      if amplifier >= amplifier_states.len() {
        amplifier_states.push((program.clone(), 0));
      }

      let state = amplifier_states.get_mut(amplifier).unwrap();
      
      let run_result;

      // Include the phase as an input when the pc is 0 (this is the first run)
      if state.1 == 0 {
        run_result = run_to_next_output(&mut state.1, &mut state.0, vec![*phases.get(amplifier).unwrap(), last_output]);
      } else {
        run_result = run_to_next_output(&mut state.1, &mut state.0, vec![last_output]);
      }

      if run_result.is_some() {
        last_output = run_result.unwrap();
      } else {
        return last_output;
      }
    }
  }
}

fn permutations(values: Vec<i32>) -> Vec<Vec<i32>> {
  if values.len() == 1 {
    return vec![values];
  }

  let mut result: Vec<Vec<i32>> = Vec::new();

  for first in &values {
    let mut remaining_values = values.clone();
    remaining_values.retain(|x| *x != *first);

    for next in permutations(remaining_values) {
      let mut new_permutation = vec![*first];
      new_permutation.extend(next);
      result.push(new_permutation);
    }
  }

  result
}

fn main() {
  let input = fs::read_to_string("data/daySeven.txt")
    .expect("Unable to read daySeven.txt");

  let input = input.split(",")
    .flat_map(str::parse::<i32>)
    .collect::<Vec<_>>();

  let mut largest_output = -1;

  for phase_setting in permutations(vec![0, 1, 2, 3, 4]) {
    let output = run_amplifiers(input.clone(), phase_setting);

    if output > largest_output {
      largest_output = output;
    }
  }

  println!("Part one: {}", largest_output);

  largest_output = 0;

  for phase_setting in permutations(vec![5, 6, 7, 8, 9]) {
    let output = run_amplifiers(input.clone(), phase_setting);

    if output > largest_output {
      largest_output = output;
    }
  }

  println!("Part two: {}", largest_output);
}