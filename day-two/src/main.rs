use std::fs;

fn main() {
    let intcode = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut opcodes: Vec<u32> = intcode.trim().split(",").map(|item| item.parse().unwrap()).collect();

    let mut current_index: usize = 0;
    loop {
      let opcode = opcodes[current_index];
      let first = opcodes[current_index + 1] as usize;
      let second = opcodes[current_index + 2 ] as usize;
      let result = opcodes[current_index + 3] as usize;

      if opcode == 1 {
        opcodes[result] = opcodes[first] + opcodes[second];
      } else if opcode == 2 {
        opcodes[result] = opcodes[first] * opcodes[second];
      } else if opcode == 99 {
        break;
      } else {
        panic!("Invalid opcode at {}: {}.", current_index, opcode);
      }

      current_index += 4;
    }

    println!("Value at index 0: {}", opcodes[0]);
}