//! Run this file with `cargo test --test 06_brainfuck_interpreter`.

// TODO (bonus): Create an interpreter for the [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) language.
// The Brainfuck program will be parsed out of a string and represented as a struct.
//
// Handle both parsing and execution errors using enums representing error conditions,
// see tests for details.
// A parsing error can be either an unknown instruction or an unpaired loop instruction.
// An execution error can be either that the program tries to read input, but there is no more
// input available, or when the program executes more than 10000 instructions (which probably
// signals an infinite loop).
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `ParseError`, `ExecuteError` and `Program`
// (and any other custom types nested inside them) so that asserts in tests work.
use core::num;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
enum ParseError {
    UnmatchedLoop { location: usize },
    UnknownInstruction { location: usize, instruction: char },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

#[derive(Debug, Eq, PartialEq)]
enum ExecuteError {
    NoInputLeft,
    InfiniteLoop,
}

impl Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ExecuteError {}

#[derive(Debug, Eq, PartialEq)]
struct Program {
    code: Vec<char>,
}

impl Program {
    fn find_corresponding_closing_bracket(&self, current_idx: usize) -> usize {
        let mut count = 0;
        let code_length = self.code.len();
        let mut idx = current_idx;

        while idx < code_length {
            let code = self.code[current_idx].to_string();

            if code == "[" {
                count += 1
            } else if code == "]" {
                count -= 1
            }

            if count < 0 {
                break;
            }

            idx += 1
        }

        return idx;
    }

    fn execute(
        &self,
        input_bytes: Vec<u8>,
        computation_bytes: Vec<u8>,
    ) -> Result<String, ExecuteError> {
        let mut num_instructions = 0;
        let mut memory = computation_bytes;
        let mut pointer = 0;
        let mut current_idx = 0;
        let mut input_idx = 0;
        let code_length = self.code.len();
        let mut output: Vec<u8> = vec![];
        let mut open_idxs = vec![];

        while current_idx < code_length {
            match self.code[current_idx].to_string().as_str() {
                "+" => {
                    memory[pointer] += 1
                }
                "-" => {
                    memory[pointer] -= 1
                }
                ">" => {
                    pointer += 1
                }
                "<" => {
                    pointer -= 1
                }
                "[" => {
                    if memory[pointer] > 0 {
                        // If the current pointer is not 0, we begin a loop and process (adding the idx of this loop start incase we need to come back)
                        open_idxs.push(current_idx - 1);
                    } else {
                        // jump to the corresponding closing bracket
                        current_idx = self.find_corresponding_closing_bracket(current_idx);
                    }
                }
                "]" => {
                    let last_open_idx = open_idxs.pop();

                    // If the current pointer is not 0, and there is an equivalent opening idx, go to that idx
                    if memory[pointer] != 0 && last_open_idx != None {
                        current_idx = last_open_idx.unwrap();
                    }
                }
                "." => {
                    output.push(memory[pointer]);
                }
                "," => {
                    if input_idx >= input_bytes.len() {
                        return Err(ExecuteError::NoInputLeft)
                    }
                    
                    memory[pointer] = input_bytes[input_idx];
                    input_idx += 1
                }
                _ => {}
            }

            current_idx += 1;
            num_instructions += 1;

            if num_instructions >= 10000 {
                return Err(ExecuteError::InfiniteLoop)
            }
        }

        Ok(String::from_utf8(output).expect("hello"))
    }
}

fn parse_program(program: &str) -> Result<Program, ParseError> {
    let allowed_commands = HashSet::from([">", "<", ".", ",", "+", "-", "[", "]"]);
    let mut stack = vec![];
    let mut last_open_bracket_idx = 0;

    for (idx, command) in program.chars().enumerate() {
        if !allowed_commands.contains(command.to_string().as_str()) {
            return Err(ParseError::UnknownInstruction {
                location: idx,
                instruction: command,
            });
        }

        match command.to_string().as_str() {
            "[" => {
                if stack.is_empty() {
                    println!("Empty stack {idx}");
                    last_open_bracket_idx = idx;
                }

                stack.push(command);
            }
            "]" => {
                let last_ele = stack.pop();
                match last_ele {
                    Some(x) => {
                        if !x.eq(&"[".chars().next().unwrap()) {
                            return Err(ParseError::UnmatchedLoop { location: idx });
                        }
                    }
                    None => return Err(ParseError::UnmatchedLoop { location: idx }),
                }
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err(ParseError::UnmatchedLoop {
            location: last_open_bracket_idx,
        });
    }

    Ok(Program {
        code: program.chars().collect(),
    })
}


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{parse_program, ExecuteError, ParseError};

    #[test]
    fn parse_empty() {
        check_output("", "", "");
    }

    #[test]
    fn parse_unknown_instruction() {
        assert!(matches!(
            parse_program(">p"),
            Err(ParseError::UnknownInstruction {
                location: 1,
                instruction: 'p'
            })
        ));
    }

    #[test]
    fn parse_unmatched_loop_start() {
        assert_eq!(
            parse_program(">++[+>][++>"),
            Err(ParseError::UnmatchedLoop { location: 7 })
        );
    }

    #[test]
    fn parse_unmatched_loop_end() {
        assert_eq!(
            parse_program(">++[+>][++>]+]"),
            Err(ParseError::UnmatchedLoop { location: 13 })
        );
    }

    #[test]
    fn missing_input() {
        let program = parse_program(",").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::NoInputLeft));
    }

    #[test]
    fn infinite_loop() {
        let program = parse_program("+[]").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::InfiniteLoop));
    }

    #[test]
    fn copy_input() {
        check_output(",.>,.>,.>,.>,.", "hello", "hello");
    }

    #[test]
    fn output_exclamation_mark() {
        check_output("+++++++++++++++++++++++++++++++++.", "", "!");
    }

    #[test]
    fn three_exclamation_marks() {
        check_output(">+++++++++++++++++++++++++++++++++<+++[>.<-]", "", "!!!");
    }

    #[test]
    fn hello_world() {
        check_output("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", "", "Hello World!\n");
    }

    fn check_output(program_text: &str, input: &str, expected_output: &str) {
        let program = parse_program(program_text);
        match program {
            Ok(program) => {
                let result = program
                    .execute(input.to_string().into_bytes(), vec![0; 30000])
                    .expect(&format!("Cannot execute program {program_text}"));
                assert_eq!(result, expected_output);
            }
            Err(error) => {
                panic!("Error occurred while parsing program {program_text}: {error:?}");
            }
        }
    }
}