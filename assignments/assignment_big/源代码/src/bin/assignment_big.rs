use failure::Error;
use serde_json;
use structopt::StructOpt;

use assignment_big::{
    helper::{
        average_coding_length, calculate_distribution, entropy, read_from_file, save_to_file,
        string_to_u8,
    },
    ArithmeticCoding, HuffmanCoding, LZCoding,
};

use cli::Command;

fn main() -> Result<(), Error> {
    let command = Command::from_args();

    match command {
        Command::LZ {
            source,
            mut target,
            mode,
            base,
            alphabet,
            prefix_code_length,
            letter_code_length,
        } => {
            let alphabet = read_from_file(&alphabet)?.chars().collect::<Vec<_>>();
            let buffer = read_from_file(&source)?;

            if mode.encode {
                let mut lz = LZCoding::new(&alphabet);

                let coded = lz.encode(&buffer, base)?;
                let coded_u8 = string_to_u8(&coded, base);
                // println!("{:?}", &coded);
                // lz.show_dictionary();

                println!(
                    "prefix_code_length: {}\tletter_code_length: {}",
                    lz.prefix_code_length, lz.letter_code_length
                );

                target.set_extension("txt");
                save_to_file(&target, coded.as_bytes())?;
                target.set_extension("data");
                save_to_file(&target, &coded_u8)?;
            } else {
                let alphabet_map = LZCoding::precoding_for_alphabet(&alphabet);

                let decoded = LZCoding::decode(
                    &buffer,
                    prefix_code_length.unwrap(),
                    letter_code_length.unwrap(),
                    &alphabet_map,
                    base,
                );
                println!("{}", &decoded);

                target.set_extension("txt");
                save_to_file(&target, decoded.as_bytes())?;
            }
        }
        Command::Huffman {
            source,
            mut target,
            mode,
            base,
            codebook,
        } => {
            let buffer = read_from_file(&source)?;

            if mode.encode {
                let distribution = calculate_distribution(&buffer);
                let codebook = HuffmanCoding::generate_mapping(&distribution, base)?;
                // println!("{:?}", &codebook);

                let avg_l = average_coding_length(&distribution, &codebook);
                let entropy = entropy(&distribution, base);
                println!("average coding length: {}\tentropy: {}", avg_l, entropy);

                let coded = HuffmanCoding::encode(&buffer, &codebook);
                let coded_u8 = string_to_u8(&coded, base);
                // println!("{:?}", &coded);

                target.set_extension("cb");
                save_to_file(&target, serde_json::to_string(&codebook)?.as_bytes())?;
                target.set_extension("txt");
                save_to_file(&target, coded.as_bytes())?;
                target.set_extension("data");
                save_to_file(target, &coded_u8)?;
            } else {
                let codebook = read_from_file(codebook.unwrap())?;
                let codebook = serde_json::from_str(&codebook)?;

                let decoded = HuffmanCoding::decode(&buffer, &codebook)?;
                println!("{}", decoded);

                target.set_extension("txt");
                save_to_file(&target, decoded.as_bytes())?;
            }
        }
        Command::Arithmetic {
            source,
            mut target,
            mode,
            distribution,
            base,
        } => {
            let buffer = read_from_file(&source)?;

            if mode.encode {
                let distribution = calculate_distribution(&buffer);

                let arith = ArithmeticCoding::new(&distribution);
                let coded = arith.encode(&buffer, base)?;
                let coded_u8 = string_to_u8(&coded, base);
                // println!("{:?}", &coded);

                target.set_extension("prob");
                save_to_file(&target, serde_json::to_string(&distribution)?.as_bytes())?;
                target.set_extension("txt");
                save_to_file(&target, coded.as_bytes())?;
                target.set_extension("data");
                save_to_file(target, &coded_u8)?;
            } else {
                let distribution = read_from_file(distribution.unwrap())?;
                let distribution = serde_json::from_str(&distribution)?;

                let arith = ArithmeticCoding::new(&distribution);
                let decoded = arith.decode(&buffer, base)?;
                println!("{}", decoded);

                target.set_extension("txt");
                save_to_file(&target, decoded.as_bytes())?;
            }
        }
    }

    Ok(())
}

mod cli {
    use std::path::PathBuf;

    use structopt::{clap::ArgGroup, StructOpt};

    #[derive(StructOpt, Debug)]
    #[structopt(group = ArgGroup::with_name("mode").required(true))]
    pub struct Mode {
        #[structopt(long, group = "mode")]
        pub encode: bool,

        #[structopt(long, group = "mode")]
        pub decode: bool,
    }

    #[derive(StructOpt, Debug)]
    #[structopt(name = env!("CARGO_PKG_NAME"))]
    #[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
    #[structopt(author = env!("CARGO_PKG_AUTHORS"))]
    pub enum Command {
        LZ {
            #[structopt(short, long, parse(from_os_str))]
            source: PathBuf,

            #[structopt(short, long, parse(from_os_str))]
            target: PathBuf,

            #[structopt(flatten)]
            mode: Mode,

            #[structopt(short, long, default_value = "2")]
            base: u32,

            #[structopt(long, parse(from_os_str))]
            alphabet: PathBuf,

            #[structopt(long, required_unless("encode"))]
            prefix_code_length: Option<u64>,

            #[structopt(long, required_unless("encode"))]
            letter_code_length: Option<u64>,
        },
        Huffman {
            #[structopt(short, long, parse(from_os_str))]
            source: PathBuf,

            #[structopt(short, long, parse(from_os_str))]
            target: PathBuf,

            #[structopt(flatten)]
            mode: Mode,

            #[structopt(short, long, default_value = "2")]
            base: u32,

            #[structopt(short, long, required_unless("encode"), parse(from_os_str))]
            codebook: Option<PathBuf>,
        },
        Arithmetic {
            #[structopt(short, long, parse(from_os_str))]
            source: PathBuf,

            #[structopt(short, long, parse(from_os_str))]
            target: PathBuf,

            #[structopt(flatten)]
            mode: Mode,

            #[structopt(short, long, required_unless("encode"), parse(from_os_str))]
            distribution: Option<PathBuf>,

            #[structopt(short, long, default_value = "2")]
            base: u32,
        },
    }
}
