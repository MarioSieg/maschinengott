use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Maschinengott",
    about = "A fast and cross-platform x86-16/32/64 disassembler."
)]
pub(crate) struct Options {
    pub input_file: PathBuf,

    #[structopt(short = "b", long = "bindump")]
    pub bin_dump: bool,

    #[structopt(short = "o", long = "out")]
    pub output_file: Option<PathBuf>,
}

impl Options {
    pub fn new() -> Self {
        Self::from_args()
    }
}
