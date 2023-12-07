use dex_checksum_tools::Dex;
use std::io::{stdin, Read};
use structopt::StructOpt;

fn main() {
	let args = Args::from_args();
	if args.debug {
		dbg!(&args);
	}
	match args.opt {
		Opt::CurrentChecksum { input_dex_file } => match input_dex_file.as_deref() {
			None | Some("-") => {
				let mut path = String::new();
				let stdin = stdin();
				let mut handle = stdin.lock();
				handle
					.read_to_string(&mut path)
					.expect("Failed to read from stdin!");
				let checksum = Dex::try_from(path.trim())
					.expect("Failed to read from stdin!")
					.current_checksum();
				println!("{:?}", checksum);
			}
			Some(path) => {
				let checksum = Dex::try_from(path)
					.expect("Failed to read from stdin!")
					.current_checksum();
				println!("{:?}", checksum);
			}
		},
		Opt::ExpectChecksum { input_dex_file } => match input_dex_file.as_deref() {
			None | Some("-") => {
				let mut path = String::new();
				let stdin = stdin();
				let mut handle = stdin.lock();
				handle
					.read_to_string(&mut path)
					.expect("Failed to read from stdin!");
				let checksum = Dex::try_from(path.trim())
					.expect("Failed to read from stdin")
					.expect_checksum();
				println!("{:?}", checksum);
			}
			Some(path) => {
				let checksum = Dex::try_from(path)
					.expect("Failed to read from stdin!")
					.expect_checksum();
				println!("{:?}", checksum);
			}
		},
		Opt::CorrectChecksum {
			input_dex_file,
			output_dex_file,
		} => {
			let correct_checksum = |input_path: &str| {
				let mut dex = Dex::try_from(input_path).expect("Failed to read from stdin!");
				let out = match output_dex_file.as_deref() {
					None => input_path,
					Some(path) => path,
				};
				if dex.correct_checksum() || out != input_path {
					dex
						.write_to_file(out)
						.expect(format!("Failed to write to {}", out).as_str());
					println!("done.")
				} else {
					println!("nothing to do.")
				}
			};

			match input_dex_file.as_deref() {
				None | Some("-") => {
					let mut path = String::new();
					let stdin = stdin();
					let mut handle = stdin.lock();
					handle
						.read_to_string(&mut path)
						.expect("Failed to read from stdin!");
					correct_checksum(path.trim());
				}
				Some(path) => {
					correct_checksum(path.trim());
				}
			}
		}
	}
}

#[derive(Debug, StructOpt)]
enum Opt {
	/// Calculates the current checksum from the DEX file's header.
	CurrentChecksum {
		/// The input dex file to read, or "-" indicating to read stdin. If omitted, stdin will be used.
		input_dex_file: Option<String>,
	},

	/// Calculates the expected checksum for the DEX file.
	ExpectChecksum {
		/// The input dex file to read, or "-" indicating to read stdin. If omitted, stdin will be used.
		input_dex_file: Option<String>,
	},

	/// Corrects the checksum in the DEX file header if it does not match the expected checksum.
	CorrectChecksum {
		/// The input dex file to read, or "-" indicating to read stdin. If omitted, stdin will be used.
		input_dex_file: Option<String>,
		/// The output file to write, If omitted, overwrites the input file.
		output_dex_file: Option<String>,
	},
}

#[derive(Debug, StructOpt)]
struct Args {
	#[structopt(subcommand)]
	opt: Opt,

	#[structopt(long, hidden = true)]
	debug: bool,
}
