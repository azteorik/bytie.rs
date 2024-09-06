use crate::context::{Context, Error};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[poise::command(slash_command)]
pub async fn brainfuck(
    ctx: Context<'_>,
    #[description = "code"] code: String,
    #[description = "inputs"] inputs_str: Option<String>,
    #[description = "ascii_codes"] ascii_codes_str: Option<String>,
) -> Result<(), Error> {
    let inputs: Vec<u8> = match (inputs_str, ascii_codes_str) {
        (Some(_), Some(_)) => {
            ctx.say("Both 'inputs' and 'ascii_codes' were provided. Please provide only one.")
                .await?;
            return Ok(());
        }
        (Some(input_str), None) => input_str.into_bytes(),
        (None, Some(ascii_codes)) => ascii_codes
            .split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .collect(),
        (None, None) => Vec::new(),
    };

    match interpret_brainfuck(code, inputs) {
        Ok(result) => {
            let output = if result.is_empty() {
                "The program produced no output.".to_string()
            } else {
                result
            };

            let truncated_output = if output.len() > 1900 {
                format!("{}... (output truncated)", &output[..1900])
            } else {
                output
            };

            ctx.say(format!("Output:\n```\n{}\n```", truncated_output))
                .await?;
        }
        Err((error, partial_output)) => {
            let output_message = if partial_output.is_empty() {
                "No output was produced before the program was terminated.".to_string()
            } else {
                format!(
                    "Partial output before termination:\n```\n{}\n```",
                    if partial_output.len() > 1800 {
                        format!("{}... (output truncated)", &partial_output[..1800])
                    } else {
                        partial_output
                    }
                )
            };
            ctx.say(format!(
                "Error executing Brainfuck code: {}\n\n{}",
                error, output_message
            ))
            .await?;
        }
    }
    Ok(())
}

const MAX_STEPS: usize = 5_000_000;
const MAX_RUNTIME: Duration = Duration::from_secs(5);

fn interpret_brainfuck(code: String, inputs: Vec<u8>) -> Result<String, (String, String)> {
    let mut memory = vec![0u8; 30000];
    let mut pointer = 0;
    let mut output = String::new();
    let mut input_queue: VecDeque<u8> = inputs.into_iter().collect();
    let mut code_pointer = 0;
    let code_chars: Vec<char> = code.chars().collect();
    let mut bracket_stack: Vec<usize> = Vec::new();

    for (i, &c) in code_chars.iter().enumerate() {
        match c {
            '[' => bracket_stack.push(i),
            ']' => {
                if bracket_stack.pop().is_none() {
                    return Err((
                        format!("Unmatched closing bracket at position {}", i),
                        output,
                    ));
                }
            }
            _ => {}
        }
    }
    if !bracket_stack.is_empty() {
        return Err((
            format!(
                "Unmatched opening bracket at position {}",
                bracket_stack.pop().unwrap()
            ),
            output,
        ));
    }

    bracket_stack.clear();

    let start_time = Instant::now();
    let mut steps = 0;
    while code_pointer < code_chars.len() {
        if start_time.elapsed() > MAX_RUNTIME {
            return Err(("Execution time limit exceeded".to_string(), output));
        }

        if steps >= MAX_STEPS {
            return Err(("Instruction limit exceeded".to_string(), output));
        }

        steps += 1;

        match code_chars[code_pointer] {
            '>' => pointer = (pointer + 1) % memory.len(),
            '<' => pointer = (pointer - 1 + memory.len()) % memory.len(),
            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            '.' => output.push(memory[pointer] as char),
            ',' => {
                if let Some(input) = input_queue.pop_front() {
                    memory[pointer] = (input & 0xFF) as u8;
                } else {
                    return Err(("Not enough input values provided".to_string(), output));
                }
            }
            '[' => {
                if memory[pointer] == 0 {
                    let mut bracket_count = 1;
                    while bracket_count > 0 {
                        code_pointer += 1;
                        if code_pointer >= code_chars.len() {
                            return Err(("Unmatched opening bracket".to_string(), output));
                        }
                        match code_chars[code_pointer] {
                            '[' => bracket_count += 1,
                            ']' => bracket_count -= 1,
                            _ => {}
                        }
                    }
                } else {
                    bracket_stack.push(code_pointer);
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    code_pointer = *bracket_stack.last().unwrap();
                } else {
                    bracket_stack.pop();
                }
            }
            _ => {}
        }
        code_pointer += 1;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let code = String::from("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
        let inputs = vec![];
        assert_eq!(
            interpret_brainfuck(code, inputs),
            Ok(String::from("Hello World!\n"))
        );
    }

    #[test]
    fn test_echo_until_null() {
        let code = String::from(",[.,]");
        let inputs = vec![72, 101, 108, 108, 111, 33, 0];
        assert_eq!(
            interpret_brainfuck(code, inputs),
            Ok(String::from("Hello!"))
        );
    }

    #[test]
    fn test_addition() {
        let code = String::from(",>,[-<+>]<.");
        let inputs = vec![3, 5];
        assert_eq!(interpret_brainfuck(code, inputs), Ok(String::from("\u{8}")));
    }

    #[test]
    fn test_insufficient_input() {
        let code = String::from(",>,<.");
        let inputs = vec![65];
        assert_eq!(
            interpret_brainfuck(code, inputs),
            Err((
                String::from("Not enough input values provided"),
                String::from("")
            ))
        );
    }

    #[test]
    fn test_unmatched_opening_bracket() {
        let code = String::from("[");
        let inputs = vec![];
        assert_eq!(
            interpret_brainfuck(code, inputs),
            Err((
                String::from("Unmatched opening bracket at position 0"),
                String::from("")
            ))
        );
    }

    #[test]
    fn test_unmatched_closing_bracket() {
        let code = String::from("]");
        let inputs = vec![];
        assert_eq!(
            interpret_brainfuck(code, inputs),
            Err((
                String::from("Unmatched closing bracket at position 0"),
                String::from("")
            ))
        );
    }
    #[test]
    fn test_nested_loops() {
        let code = String::from("++++[>++[>+<-]<-]>>.");
        let inputs = vec![];
        assert_eq!(interpret_brainfuck(code, inputs), Ok(String::from("\u{8}")));
    }
}
