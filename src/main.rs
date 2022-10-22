mod code;

use clap::{Parser, Subcommand};
use code::{decode, encode, Bits};

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

fn main() {
   let args = Cli::parse();
   match args.command {
      Commands::Encode { plane } => {
         let code = encode(&plane);
         let bits: String = code
            .into_iter()
            .map(|x| if x { "1" } else { "0" })
            .collect();
         println!("{}", bits);
      }
      Commands::Decode { bit_string } => {
         let bits = Bits::from_iter(bit_string.chars().filter(|&x| x == '1' || x == '0').map(
            |x| match x {
               '1' => true,
               '0' => false,
               _ => unreachable!(),
            },
         ));
         let plane = decode(&bits);
         println!("{}", plane);
      }
   }
}
