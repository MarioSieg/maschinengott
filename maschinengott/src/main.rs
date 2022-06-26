mod disassembler;
mod options;
mod pe;

fn main() {
    let options = options::Options::new();

    println!("Reading imagine: {:?}...", options.input_file);
    let (code, rip) = pe::read_machine_code(&options.input_file);

    println!("Disassembling machine code...");
    let data = disassembler::disassemble(&code, disassembler::Bitness::X64, rip, options.bin_dump);

    println!("Writing assembly...");
    if let Some(output_file) = options.output_file {
        let mut output = String::new();
        for line in data {
            output += &line;
        }
        std::fs::write(output_file, output).expect("Failed to write output file!");
    } else {
        for line in data {
            println!("{}", line);
        }
    }
}
