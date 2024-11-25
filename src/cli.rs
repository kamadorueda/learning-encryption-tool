use std::path::PathBuf;

use clap::value_parser;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use rpassword::prompt_password;

#[derive(Debug, Parser)]
#[clap(name = "crypt", term_width = 80, version)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Encrypt {
        /// Input file.
        #[arg(long, short, value_parser = value_parser!(PathBuf))]
        in_: PathBuf,

        /// Output file.
        #[arg(long, short, value_parser = value_parser!(PathBuf))]
        out: PathBuf,

        /// Algorithm to use
        #[arg(long, short, value_enum, default_value_t = Alg::AesGcm)]
        alg: Alg,
    },
    Decrypt {
        /// Input file.
        #[arg(long, short, value_parser = value_parser!(PathBuf))]
        in_: PathBuf,

        /// Output file.
        #[arg(long, short, value_parser = value_parser!(PathBuf))]
        out: PathBuf,
    },
    Show {
        /// Input file.
        #[arg(long, short, value_parser = value_parser!(PathBuf))]
        in_: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Alg {
    AesGcm,
}

pub fn cli() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Encrypt { in_, out, .. } => {
            let password = prompt_password("password > ")?;

            crate::actions::encrypt::encrypt(password.as_bytes(), in_, out)
        }
        Commands::Decrypt { in_, out } => {
            let password = prompt_password("password > ")?;

            crate::actions::decrypt::decrypt(password.as_bytes(), in_, out)
        }
        Commands::Show { in_ } => crate::actions::show::show(in_),
    }
}
