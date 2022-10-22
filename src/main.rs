mod code;

use std::{error, fmt};

use clap::{Parser, Subcommand};
use code::{decode, encode, Bits};

#[derive(Debug, PartialEq)]
enum CliError {
   InvalidPlaneText,
   InvalidBitString,
}

impl fmt::Display for CliError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self {
         CliError::InvalidPlaneText => write!(f, "Invalid plane text: detected char except ascii."),
         CliError::InvalidBitString => {
            write!(f, "Invalid bit string: detected char except '1' and '0'.")
         }
      }
   }
}

impl error::Error for CliError {
   fn source(&self) -> Option<&(dyn error::Error + 'static)> {
      None
   }
}

#[derive(Debug, Parser)]
#[command(name = "huffman")]
#[command(about = "Encode/Decode text by huffman coding.", long_about=None)]
struct Cli {
   #[command(subcommand)]
   command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
   /// Encode plane text.
   #[command()]
   Encode {
      #[arg()]
      plane: String,
   },
   /// Decode bit string.
   #[command()]
   Decode {
      #[arg()]
      bit_string: String,
   },
}

fn handle_error(error: CliError) -> CliError {
   println!("{}", error);
   error
}

fn main() -> Result<(), CliError> {
   let args = Cli::parse();
   Ok(match args.command {
      Commands::Encode { plane } => {
         if !plane.is_ascii() {
            return Err(handle_error(CliError::InvalidPlaneText));
         }
         let code = encode(&plane);
         let bits: String = code
            .into_iter()
            .map(|x| if x { "1" } else { "0" })
            .collect();
         println!("{}", bits);
      }
      Commands::Decode { bit_string } => {
         let mut validated_bit_string = bit_string.chars().map(|x| match x {
            '1' => Ok(true),
            '0' => Ok(false),
            _ => Err(CliError::InvalidBitString),
         });
         if validated_bit_string.any(|x| x == Err(CliError::InvalidBitString)) {
            return Err(handle_error(CliError::InvalidBitString));
         }
         let bits = Bits::from_iter(bit_string.chars().map(|x| match x {
            '1' => true,
            '0' => false,
            _ => unreachable!(),
         }));
         let plane = decode(&bits);
         println!("{}", plane);
      }
   })
}
