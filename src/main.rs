mod instruction_generator;
mod parser;
mod tests;

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .expect("First argument must be the input file");
    let output_file = std::env::args()
        .nth(2)
        .expect("Second argument must be the output file");

    let input_str = std::fs::read_to_string(input_file).expect("Input file doesn't exist");

    let binary_u16 = parser::parse(&input_str);
    let mut binary_u8 = Vec::<u8>::new();

    for n in binary_u16 {
        binary_u8.push((n >> 8) as u8);
        binary_u8.push((n & 0xFF) as u8);
    }

    std::fs::write(output_file, binary_u8).expect("Can't create output file");
}
