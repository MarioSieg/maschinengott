use iced_x86::{Decoder, DecoderOptions, Formatter, GasFormatter, Instruction};
use linya::{Bar, Progress};
use rayon::prelude::*;
use std::sync::Mutex;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Bitness {
    X16 = 16,
    X32 = 32,
    X64 = 64,
}

pub fn disassemble(bytes: &[u8], bitness: Bitness, rip: u64, bin: bool) -> Vec<String> {
    let width = if bin { 64 + 16 } else { 32 };

    let instructions: Vec<Instruction> = {
        let mut decoder = Decoder::with_ip(bitness as u32, bytes, rip, DecoderOptions::NONE);
        decoder.iter().collect()
    };

    let result = instructions
        .par_iter()
        .map(|&instruction| {
            let mut out = String::new();
            let mut formatter = GasFormatter::new();
            let options = formatter.options_mut();
            options.set_uppercase_mnemonics(false);
            options.set_gas_show_mnemonic_size_suffix(true);
            options.set_first_operand_char_index(8);
            options.set_space_after_memory_bracket(true);
            options.set_space_after_operand_separator(true);
            options.set_always_show_segment_register(true);
            formatter.format(&instruction, &mut out);

            let mut line = if bin {
                format!("{:016b} ", instruction.ip())
            } else {
                format!("{:016X} ", instruction.ip())
            };

            let mut machine_code = String::new();
            let start_index = (instruction.ip() - rip) as usize;
            let instr_bytes = &bytes[start_index..start_index + instruction.len()];
            for b in instr_bytes.iter() {
                if bin {
                    machine_code = format!("{}{:08b} ", machine_code, b);
                } else {
                    machine_code = format!("{}{:02X} ", machine_code, b);
                }
            }

            line = format!(
                "{} | {:0width$} | {:<32} | {}",
                line,
                machine_code,
                instruction.op_code().op_code_string(),
                out,
                width = width
            );
            line
        })
        .collect::<Vec<String>>();

    result
}
