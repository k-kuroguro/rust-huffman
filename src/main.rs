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

fn execute_command(args: Cli) -> Result<String, CliError> {
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
         bits
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
         plane
      }
   })
}

fn main() -> Result<(), CliError> {
   let args = Cli::parse();
   match execute_command(args) {
      Ok(x) => Ok(println!("{}", x)),
      Err(x) => Err(x),
   }
}

#[cfg(test)]
mod tests {
   use super::{execute_command, Cli, CliError, Commands};

   static LOREM_IPSUM_PLANE: &str =
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
      tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
      veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea \
      commodo consequat. Duis aute irure dolor in reprehenderit in voluptate \
      velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
      cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id \
      est laborum.";

   static LOREM_IPSUM_CODE: &str =
      "000000000000000000000111001101110000000000000000000000011011110100000000000000000\
         0000001000110001001011001011100010101110000010110110001011010010101110010101101110\
         0101001100101000101010111100001011101101011001110010110011010110001001010001001011\
         0100010101010100100101110000101110101000101101111101100001010111010010110001110110\
         1101000100100000101110011101100100010110010100000011000001000010011111011111000100\
         0011110001111011111011101100000011000010011011100001101011010011011111111010000000\
         1101011010000101111001111101101010111110100111010011010011110100100001001111001011\
         0001010101100111101111000100110100000001101110011111110111011101100011011110010111\
         1110010111100011101110101011111011100001100001001100010101101100011110100111101011\
         1010110101100111101011000011001011010110000100111111011111010110111011000000110000\
         1001111110101111001011001101011001110100100010010000010111100101101111100110110111\
         0101101111010100110111110100111101110101110010101001101111100110010111101010011001\
         1011100000011000000101110011110011001011000111001010010001111110111011110110001111\
         1010010110001101010011010001100001011100111000100011001101111011010001100001100101\
         1010110000100001111001100101001111000011100111101011010010001001000001011100100001\
         1101111011000111011111001110101101000101111011110001110110001101011010000101111001\
         1110000010111100110100110111110011011000111001111001101001011110101111110001010001\
         1101001111110111011000000110000100110001010111001001111000010100111101101101011110\
         1011110111110100001101011000101011100110010100000010111000011010100110101111110011\
         0010111100010011010110111111100111001111110101100010001000101111011111011101100000\
         0110000100111111011110111110011010001110110011001100110101100101011100010001100111\
         0000011001010000110011010011101000110111110011000010110001101101111000011010111101\
         1101001101110000101011010110100010110101101001111110110100110101101011001110000100\
         1111011001101010011010110010110000101110000010100100000111101111101011010000000110\
         1110001110101101011000101011101011001110001000011001110000001011100111010000110100\
         0110100001101100011001110111011111111001111010001110101101011010111100000010001001\
         1010110100101010011011111000111101110111111100101011000011001011010110000100011110\
         1110110111";

   #[test]
   fn encoding_test() {
      let arg = Cli {
         command: Commands::Encode {
            plane: String::from(LOREM_IPSUM_PLANE),
         },
      };
      let expected_bits = String::from(LOREM_IPSUM_CODE);
      assert_eq!(expected_bits, execute_command(arg).unwrap());
   }

   #[test]
   fn decoding_test() {
      let arg = Cli {
         command: Commands::Decode {
            bit_string: String::from(LOREM_IPSUM_CODE),
         },
      };
      let expected_plane = String::from(LOREM_IPSUM_PLANE);
      assert_eq!(expected_plane, execute_command(arg).unwrap());
   }

   #[test]
   fn encoding_error_test() {
      let arg = Cli {
         command: Commands::Encode {
            plane: String::from("あいうえお🌳"),
         },
      };
      assert_eq!(
         CliError::InvalidPlaneText,
         execute_command(arg).unwrap_err()
      );
   }

   #[test]
   fn decoding_error_test() {
      let arg = Cli {
         command: Commands::Decode {
            bit_string: String::from("1010105"),
         },
      };
      assert_eq!(
         CliError::InvalidBitString,
         execute_command(arg).unwrap_err()
      );
   }
}
