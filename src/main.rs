#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]

mod instruction_generator;
mod parser;
mod tests;

fn main() {
    if let Some(input_file) = std::env::args().nth(1) {
        if let Some(output_file) = std::env::args().nth(2) {
            match std::fs::read_to_string(input_file) {
                Ok(input_str) => match parser::parse(&input_str) {
                    Ok(binary_u16) => {
                        let mut binary_u8 = Vec::<u8>::new();

                        for n in binary_u16 {
                            binary_u8.push((n >> 8) as u8);
                            binary_u8.push((n & 0xFF) as u8);
                        }

                        match std::fs::write(output_file, binary_u8) {
                            Ok(()) => {
                                println!("Assembly successfull !");
                            }

                            Err(write_error) => {
                                println!("Error, can't write output file : {write_error}");
                            }
                        }
                    }

                    Err(parse_error) => {
                        println!("{parse_error}");
                    }
                },
                Err(read_error) => {
                    println!("Error can't read input file : {read_error}");
                }
            }
        } else {
            println!("Error need output file as second argument");
        }
    } else {
        println!("Error needs input file as first argument");
    }
}
