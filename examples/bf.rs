use std::{
    env,
    io::{self, Read},
};
use wrapnum::{wrap, WrapNum};

fn main() {
    let mut memory: Vec<WrapNum<u32>> = vec![wrap!(255); 30_000];
    let mut pointer: WrapNum<usize> = wrap!(0, 29_999);

    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        eprintln!("No user input passed");
        std::process::exit(1);
    }

    let code: Vec<char> = args[1].chars().collect();

    let mut pc = 0;
    let mut loop_stack = Vec::new();

    while pc < code.len() {
        match code[pc] {
            '>' => pointer += 1,
            '<' => {
                println!("{}", pointer);
                pointer -= 1
            }
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '.' => print!("{}", char::from_u32(memory[pointer].value).unwrap()),
            ',' => {
                let mut buf = [0u8];
                if io::stdin().read_exact(&mut buf).is_ok() {
                    memory[pointer] = wrap!(buf[0] as u32, 0, 255);
                }
            }
            '[' => {
                if memory[pointer].value == 0 {
                    let mut unmatched = 1;
                    while unmatched > 0 {
                        pc += 1;
                        if pc >= code.len() {
                            break;
                        }
                        match code[pc] {
                            '[' => unmatched += 1,
                            ']' => unmatched -= 1,
                            _ => {}
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            }
            ']' => {
                if memory[pointer].value != 0 {
                    if let Some(&loop_start) = loop_stack.last() {
                        pc = loop_start - 1; // -1 because we'll increment it below.
                    }
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        pc += 1;
    }
}
