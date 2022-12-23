use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Maschinengott",
    about = "A fast and cross-platform x86-64 disassembler."
)]
pub(crate) struct Options {
    pub input_file: PathBuf, // the input image file

    #[structopt(short = "m", long = "most_used")]
    pub max_hot_instructions: Option<usize>, // print this amount of most used instructions

    #[structopt(short = "b", long = "bin_dump")]
    pub bin_dump: bool, // dump addresses and machine code in binary instead in hex

    #[structopt(short = "d", long = "dis_asm")]
    pub disassemble: bool, // disassembler the machine code

    #[structopt(long = "intel_syntax")]
    pub use_intel_syntax: bool, // use Intel syntax instead of AT&T in formatting

    #[structopt(short = "o", long = "out")]
    pub output_file: Option<PathBuf>, // the output file to write the assembly into
}

impl Options {
    pub fn new() -> Self {
        Self::from_args()
    }
}
