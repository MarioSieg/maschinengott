mod disassembler;
mod options;
mod pe;

use indicatif::HumanBytes;
use num_format::{Locale, ToFormattedString};
use std::time::Instant;

fn main() {
    let clock = Instant::now();
    let options = options::Options::new();

    println!("Reading image: {:?}...", options.input_file);
    let (code, rip, image_size) = pe::read_machine_code(&options.input_file);

    println!("Disassembling machine code...");
    let data = disassembler::disassemble(
        &code,
        disassembler::Bitness::X64,
        rip,
        options.bin_dump,
        options.use_intel_syntax,
    );

    if options.disassemble {
        println!("Writing assembly...");
        if let Some(output_file) = options.output_file {
            let mut output = String::new();
            for line in &data.assembly {
                output += line;
            }
            std::fs::write(output_file, output).expect("Failed to write output file!");
        } else {
            for line in &data.assembly {
                print!("{}", line);
            }
        }
    }

    println!(
        "-- Summary of {:?} --",
        options.input_file.file_name().unwrap_or_default()
    );
    println!(
        "Total instructions: {}",
        data.assembly.len().to_formatted_string(&Locale::en)
    );
    println!("Image size: {}", HumanBytes(image_size));
    print!("\nISA Extensions used: ");
    for (i, ex) in data.isa_extensions_used.iter().enumerate() {
        if i % 8 == 0 {
            print!("\n\t");
        }
        print!("{}, ", ex);
    }
    println!("\n");

    let max = options.max_hot_instructions.unwrap_or(32);
    println!("The {} most used instructions:", max);
    for (i, (k, v)) in data.most_used_instructions.iter().enumerate() {
        println!(
            "\t{:<16} | {} usages",
            k,
            v.to_formatted_string(&Locale::en)
        );
        if i > max {
            break;
        }
    }

    println!("Finished in {} s", clock.elapsed().as_secs_f64());
}
