#![allow(dead_code)]

use std::env::args;
use std::fs::read;
use std::path::Path;
use std::time::Instant;

mod interpreters {
    const MEMORY_SIZE: usize = 2usize.pow(15);

    pub mod basic {
        use std::collections::HashMap;
        use std::io::{stdin, stdout, BufRead, BufWriter, StdoutLock, Write};

        use crate::interpreters::MEMORY_SIZE;

        fn generate_bracket_map(program: &[u8]) -> Result<HashMap<usize, usize>, &'static str> {
            let mut stack: Vec<usize> = Vec::new();
            let mut bracket_map: HashMap<usize, usize> = HashMap::new();

            for (i, instruction) in program.iter().enumerate() {
                match instruction {
                    b'[' => {
                        stack.push(i);
                    }
                    b']' => {
                        if let Some(opening_bracket_index) = stack.pop() {
                            bracket_map.insert(opening_bracket_index, i);
                            bracket_map.insert(i, opening_bracket_index);
                        } else {
                            return Err("Unbalanced brackets in program.");
                        }
                    }
                    _ => {}
                };
            }

            if stack.is_empty() {
                Ok(bracket_map)
            } else {
                Err("Unbalanced brackets in program.")
            }
        }

        pub fn run(program: &[u8]) {
            if program.is_empty() {
                return;
            }

            let mut buffered_stdout_lock: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());

            let mut memory: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];
            let mut memory_pointer: usize = 0usize;
            let mut instruction_pointer: usize = 0usize;
            let mut input_buffer: Vec<u8> = Vec::new();

            let bracket_map: HashMap<usize, usize> = generate_bracket_map(&program).unwrap();

            while let Some(&instruction) = program.get(instruction_pointer) {
                match instruction {
                    b'>' => memory_pointer += 1,
                    b'<' => memory_pointer -= 1,
                    b'+' => {
                        memory[memory_pointer] = memory[memory_pointer].wrapping_add(1);
                    }
                    b'-' => {
                        memory[memory_pointer] = memory[memory_pointer].wrapping_sub(1);
                    }
                    b'.' => {
                        if let Some(byte) = memory
                            .get(memory_pointer)
                            .cloned()
                            .filter(|&byte| byte.is_ascii())
                        {
                            buffered_stdout_lock.write_all(&[byte]).unwrap();
                            buffered_stdout_lock.flush().unwrap();
                        }
                    }
                    b',' => {
                        if input_buffer.is_empty() {
                            stdin().lock().read_until(b'\n', &mut input_buffer).unwrap();
                        }
                        memory[memory_pointer] = input_buffer.remove(0);
                    }
                    b'[' => {
                        if memory[memory_pointer] == 0 {
                            instruction_pointer = *bracket_map.get(&instruction_pointer).unwrap();
                        }
                    }
                    b']' => {
                        if memory[memory_pointer] != 0 {
                            instruction_pointer = *bracket_map.get(&instruction_pointer).unwrap();
                        }
                    }
                    _ => {}
                }
                instruction_pointer += 1;
            }
        }
    }

    pub mod optimized {
        use std::collections::HashMap;
        use std::io::{stdin, stdout, BufRead, BufWriter, StdoutLock, Write};

        use crate::interpreters::MEMORY_SIZE;

        fn compress_program(program: &[u8]) -> Vec<(u8, usize)> {
            let mut compressed_program: Vec<(u8, usize)> = Vec::with_capacity(program.len());
            let Some((program_start_index, mut current_instruction)) = program
                .iter()
                .enumerate()
                .find(|(_, &instruction)| {
                    if let b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']' = instruction {
                        true
                    } else {
                        false
                    }
                }) else { return compressed_program; };
            let mut instruction_count: usize = 1usize;

            for instruction in program.iter().skip(program_start_index + 1usize) {
                if let b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']' = instruction {
                    if let b'>' | b'<' | b'+' | b'-' = current_instruction {
                        if current_instruction == instruction {
                            instruction_count += 1usize;
                            continue;
                        };
                    };
                    compressed_program.push((*current_instruction, instruction_count));
                    current_instruction = instruction;
                    instruction_count = 1usize;
                };
            }

            compressed_program.push((*current_instruction, instruction_count));

            compressed_program
        }

        fn generate_bracket_map(
            compressed_program: &[(u8, usize)],
        ) -> Result<HashMap<usize, usize>, &'static str> {
            let mut stack: Vec<usize> = Vec::new();
            let mut bracket_map: HashMap<usize, usize> = HashMap::new();

            for (i, (instruction, _)) in compressed_program.iter().enumerate() {
                match instruction {
                    b'[' => {
                        stack.push(i);
                    }
                    b']' => {
                        if let Some(opening_bracket_index) = stack.pop() {
                            bracket_map.insert(opening_bracket_index, i);
                            bracket_map.insert(i, opening_bracket_index);
                        } else {
                            return Err("Unbalanced brackets in program.");
                        }
                    }
                    _ => {}
                };
            }

            if stack.is_empty() {
                Ok(bracket_map)
            } else {
                Err("Unbalanced brackets in program.")
            }
        }

        pub fn run(program: &[u8]) {
            if program.is_empty() {
                return;
            }
            let mut buffered_stdout_lock: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());

            let mut memory: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];
            let mut memory_pointer: usize = 0usize;
            let mut instruction_pointer: usize = 0usize;
            let mut input_buffer: Vec<u8> = Vec::new();

            let compressed_program: Vec<(u8, usize)> = compress_program(&program);
            let bracket_map: HashMap<usize, usize> =
                generate_bracket_map(&compressed_program).unwrap();

            while let Some(&(instruction, instruction_count)) =
                compressed_program.get(instruction_pointer)
            {
                match instruction {
                    b'>' => memory_pointer += instruction_count,
                    b'<' => memory_pointer -= instruction_count,
                    b'+' => {
                        memory[memory_pointer] = memory[memory_pointer]
                            .wrapping_add((instruction_count % 256usize) as u8);
                    }
                    b'-' => {
                        memory[memory_pointer] = memory[memory_pointer]
                            .wrapping_sub((instruction_count % 256usize) as u8);
                    }
                    b'.' => {
                        if let Some(byte) = memory
                            .get(memory_pointer)
                            .cloned()
                            .filter(|&byte| byte.is_ascii())
                        {
                            buffered_stdout_lock.write_all(&[byte]).unwrap();
                            buffered_stdout_lock.flush().unwrap();
                        }
                    }
                    b',' => {
                        if input_buffer.is_empty() {
                            stdin().lock().read_until(b'\n', &mut input_buffer).unwrap();
                        }
                        memory[memory_pointer] = input_buffer.remove(0);
                    }
                    b'[' => {
                        if memory[memory_pointer] == 0 {
                            instruction_pointer = *bracket_map.get(&instruction_pointer).unwrap();
                        }
                    }
                    b']' => {
                        if memory[memory_pointer] != 0 {
                            instruction_pointer = *bracket_map.get(&instruction_pointer).unwrap();
                        }
                    }
                    _ => {}
                }
                instruction_pointer += 1;
            }
        }
    }
}

fn main() {
    let program: Vec<u8> = read(Path::new(
        &args()
            .into_iter()
            .nth(1)
            .expect("A .b source file is required as an argument."),
    ))
    .unwrap();

    let start: Instant = Instant::now();
    interpreters::optimized::run(&program);
    println!("\nExecution finished in {:?}", start.elapsed());
}
