use clap::Parser;
use anyhow::Result;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};

mod lnk_file;
mod lnk_target_path;
use lnk_file::*;

#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
struct Cli {
    /// Name of the LNK files to read from
    pub(crate) lnk_files: Vec<String>,

    #[command(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    for filename in cli.lnk_files {
        match LnkFile::try_from(&filename[..]) {
            Ok(lnk_file) => lnk_file.print_bodyfile(),
            Err(why) => log::error!("unable to open {filename}: {why}"),
        }
    }

    Ok(())
}
