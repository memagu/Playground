use clap::{Arg, ArgMatches, Command, ValueHint};

use std::fs::read;
use std::path::{Path, PathBuf};

const MEMORY_SIZE: usize = 2usize.pow(15);

mod interpreters {
    pub mod basic {
        use std::collections::HashMap;
        use std::io::{stdin, stdout, BufRead, BufWriter, StdoutLock, Write};

        use crate::MEMORY_SIZE;

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

        use crate::MEMORY_SIZE;

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
                            .wrapping_add((instruction_count % 256usize) as u8)
                    }
                    b'-' => {
                        memory[memory_pointer] = memory[memory_pointer]
                            .wrapping_sub((instruction_count % 256usize) as u8)
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

mod compilers {
    const RUST_INTERMEDIARY_FILENAME: &str = "transpiled_brainfuck.rs";

    pub mod optimized {
        use std::fs::{remove_file, File};
        use std::io::Write;
        use std::path::PathBuf;
        use std::process::Command;

        use crate::compilers::RUST_INTERMEDIARY_FILENAME;
        use crate::MEMORY_SIZE;

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

        fn transpile(compressed_program: &Vec<(u8, usize)>) -> Vec<u8> {
            let mut transpiled_program: Vec<u8> = Vec::new();

            transpiled_program.extend(format!("use std::io::{{stdin, stdout, BufRead, BufWriter, StdoutLock, Write}};const MEMORY_SIZE: usize = {}usize;fn main () {{let mut buffered_stdout_lock: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());let mut memory: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];let mut memory_pointer: usize = 0usize;let mut input_buffer: Vec<u8> = Vec::new();", MEMORY_SIZE).bytes());
            for (instruction, instruction_count) in compressed_program {
                transpiled_program.extend(match instruction {
                    b'>' => format!("memory_pointer += {}usize;", instruction_count),
                    b'<' => format!("memory_pointer -= {}usize;", instruction_count),
                    b'+' => format!("memory[memory_pointer] = memory[memory_pointer].wrapping_add(({}usize % 256usize) as u8);", instruction_count),
                    b'-' => format!("memory[memory_pointer] = memory[memory_pointer].wrapping_sub(({}usize % 256usize) as u8);", instruction_count),
                    b'.' => "if let Some(byte) = memory.get(memory_pointer).cloned().filter(|&byte| byte.is_ascii()){buffered_stdout_lock.write_all(&[byte]).unwrap();buffered_stdout_lock.flush().unwrap();}".to_string(),
                    b',' => "if input_buffer.is_empty() {stdin().lock().read_until(b'\\n', &mut input_buffer).unwrap();};memory[memory_pointer] = input_buffer.remove(0usize);".to_string(),
                    b'[' => "while *memory.get(memory_pointer).unwrap() != 0u8 {".to_string(),
                    b']' => "};".to_string(),
                    _ => String::new(),
                }.bytes()
                )
            }

            transpiled_program.push(b'}');

            transpiled_program
        }

        pub fn compile(program: &[u8], output_path: &String) {
            File::create(RUST_INTERMEDIARY_FILENAME)
                .unwrap()
                .write_all(&transpile(&compress_program(&program)))
                .unwrap();

            Command::new("rustc")
                .args([
                    RUST_INTERMEDIARY_FILENAME,
                    "-o",
                    output_path,
                    "-C",
                    "opt-level=3",
                    "-C",
                    "debuginfo=0",
                ])
                .output()
                .expect("Program failed to compile.");

            remove_file(RUST_INTERMEDIARY_FILENAME).unwrap();

            let mut path_buf: PathBuf = PathBuf::from(output_path);
            path_buf.set_extension("pdb");

            if path_buf.exists() {
                remove_file(path_buf).unwrap();
            };
        }
    }
}

fn main() {
    let cmd: Command = Command::new("brainfuck")
        .version("1.0.0")
        .author("Melker Widen")
        .about("A bundled interpreter and compiler for brainfuck programs.")
        .propagate_version(true)
        .subcommand_required(true)
        .arg(
            Arg::new("INPUT")
                .value_name("INPUT")
                .value_hint(ValueHint::FilePath)
                .help("Brainfuck source file.")
                .required(true)
                .index(1usize),
        )
        .subcommand(
            Command::new("interpreter")
                .short_flag('I')
                .about("Use as an interpreter")
                .arg(
                    Arg::new("mode")
                        .short('m')
                        .long("mode")
                        .value_parser(["basic", "optimized"])
                        .default_value("optimized")
                        .help("Set optimization level of interpreter"),
                ),
        )
        .subcommand(
            Command::new("compiler")
                .short_flag('C')
                .about("Use as a compiler")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILENAME")
                        .value_hint(ValueHint::FilePath)
                        .help("Set output path of compiled brainfuck program"),
                ),
        );

    let matches: ArgMatches = cmd.get_matches();

    let input: &String = matches.get_one::<String>("INPUT").unwrap();

    let brainfuck_program: Vec<u8> = read(Path::new(input)).unwrap();

    match matches.subcommand() {
        Some(("interpreter", sub_matches)) => {
            match sub_matches.get_one::<String>("mode").unwrap().as_str() {
                "basic" => interpreters::basic::run(&brainfuck_program),
                "optimized" => interpreters::optimized::run(&brainfuck_program),
                _ => panic!("Invalid mode. Use 'basic' or 'optimized'."),
            };
        }
        Some(("compiler", sub_matches)) => {
            let output_path: String = sub_matches
                .get_one::<String>("output")
                .cloned()
                .unwrap_or_else(|| -> String {
                    let mut path_buf: PathBuf = PathBuf::from(input);
                    path_buf.set_extension("exe");
                    path_buf.file_name().unwrap().to_string_lossy().into_owned()
                });
            compilers::optimized::compile(&brainfuck_program, &output_path);
        }
        _ => (),
    }
}
