use bitvec::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub type Bits = BitVec<u8, Msb0>;
type Weight = u64;
type CharBitsMap = HashMap<u8, Bits>;
type BitsCharMap = HashMap<Bits, u8>;

struct Code {
   compressed_char_count: u32,
   original_char_count: u32,
   tree_topology_size: u32,
   tree_topology: Bits,
   compressed_chars: Bits,
}

impl From<&Bits> for Code {
   fn from(item: &Bits) -> Self {
      fn to_u32(x: &[u8; 4]) -> u32 {
         (x[0] as u32) << 24 | (x[1] as u32) << 16 | (x[2] as u32) << 8 | x[3] as u32
      }

      let raw_slice = item.as_raw_slice();
      let compressed_char_count = to_u32(raw_slice[0..4].try_into().unwrap());
      let original_char_count = to_u32(raw_slice[4..8].try_into().unwrap());
      let tree_topology_size = to_u32(raw_slice[8..12].try_into().unwrap());

      let tree_topology_start_index: usize = 96;
      let tree_topology_end_index = tree_topology_start_index + tree_topology_size as usize;
      let compressed_chars_start_index = tree_topology_end_index;

      let tree_topology =
         BitVec::from_bitslice(&item[tree_topology_start_index..tree_topology_end_index]);
      let compressed_chars = BitVec::from_bitslice(&item[compressed_chars_start_index..]);

      Code {
         compressed_char_count,
         original_char_count,
         tree_topology_size,
         tree_topology,
         compressed_chars,
      }
   }
}

impl Code {
   fn to_bits(&self) -> Bits {
      fn to_u8_array(x: &u32) -> [u8; 4] {
         [
            ((x >> 24) & 0xff) as u8,
            ((x >> 16) & 0xff) as u8,
            ((x >> 8) & 0xff) as u8,
            (x & 0xff) as u8,
         ]
      }

      let mut result = Bits::new();
      result.extend(to_u8_array(&self.compressed_char_count));
      result.extend(to_u8_array(&self.original_char_count));
      result.extend(to_u8_array(&self.tree_topology_size));
      result.extend(self.tree_topology.clone());
      result.extend(self.compressed_chars.clone());
      result
   }
}

struct Tree {
   root: Box<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Leaf {
   char: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Branch {
   right: Box<Node>,
   left: Box<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
   Branch(Branch),
   Leaf(Leaf),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HeapData {
   weight: Reverse<Weight>,
   node: Box<Node>,
}

fn evaluate_weight(plane: &[u8]) -> BinaryHeap<HeapData> {
   let mut heap: BinaryHeap<HeapData> = BinaryHeap::new();
   let unique_chars: HashSet<&u8> = plane.iter().collect();
   for &char in unique_chars {
      heap.push(HeapData {
         weight: Reverse(plane.iter().filter(|x| **x == char).count() as Weight),
         node: Box::new(Node::Leaf(Leaf { char })),
      });
   }
   heap
}

fn generate_tree(heap: &mut BinaryHeap<HeapData>) -> Tree {
   loop {
      if heap.len() <= 1 {
         break;
      }

      let left = heap.pop().unwrap();
      let right = heap.pop().unwrap();

      let branch = Node::Branch(Branch {
         left: left.node,
         right: right.node,
      });
      heap.push(HeapData {
         weight: Reverse(left.weight.0.saturating_add(right.weight.0)),
         node: Box::new(branch),
      });
   }

   Tree {
      root: heap.pop().unwrap().node,
   }
}

fn generate_char_bits_map(tree: &Tree) -> CharBitsMap {
   fn search_by_dfs_nlr(node: Box<Node>, char_bits_map: &mut CharBitsMap, bits: &mut Bits) {
      match *node {
         Node::Branch(x) => {
            let mut left_bits = bits.clone();
            let mut right_bits = bits.clone();

            left_bits.push(false);
            right_bits.push(true);

            search_by_dfs_nlr(x.left, char_bits_map, &mut left_bits);
            search_by_dfs_nlr(x.right, char_bits_map, &mut right_bits);
         }
         Node::Leaf(x) => {
            if bits.len() == 0 {
               bits.push(false);
            }
            char_bits_map.insert(x.char, bits.clone());
         }
      }
   }

   let mut char_bits_map: CharBitsMap = HashMap::new();
   let mut bits = Bits::new();
   search_by_dfs_nlr(tree.root.clone(), &mut char_bits_map, &mut bits);
   char_bits_map
}

fn generate_bits_char_map(tree: &Tree) -> BitsCharMap {
   fn search_by_dfs_nlr(node: Box<Node>, bits_char_map: &mut BitsCharMap, bits: &mut Bits) {
      match *node {
         Node::Branch(x) => {
            let mut left_bits = bits.clone();
            let mut right_bits = bits.clone();

            left_bits.push(false);
            right_bits.push(true);

            search_by_dfs_nlr(x.left, bits_char_map, &mut left_bits);
            search_by_dfs_nlr(x.right, bits_char_map, &mut right_bits);
         }
         Node::Leaf(x) => {
            if bits.len() == 0 {
               bits.push(false);
            }
            bits_char_map.insert(bits.clone(), x.char);
         }
      }
   }

   let mut bits_char_map: BitsCharMap = HashMap::new();
   let mut bits = Bits::new();
   search_by_dfs_nlr(tree.root.clone(), &mut bits_char_map, &mut bits);
   bits_char_map
}

fn encode_tree(tree: &Tree) -> Bits {
   fn search_by_dfs_lrn(node: Box<Node>, bits: &mut Bits) {
      match *node {
         Node::Branch(x) => {
            search_by_dfs_lrn(x.left, bits);
            search_by_dfs_lrn(x.right, bits);
            bits.push(false);
         }
         Node::Leaf(x) => {
            bits.push(true);
            bits.extend(&[x.char]);
         }
      }
   }

   let mut tree_topology = Bits::new();
   search_by_dfs_lrn(tree.root.clone(), &mut tree_topology);
   tree_topology.push(false);
   tree_topology
}

pub fn encode(plane: &String) -> Bits {
   let ascii_array = plane.as_bytes();
   let tree = generate_tree(&mut evaluate_weight(ascii_array));
   let char_bits_map = generate_char_bits_map(&tree);
   let mut compressed_chars = Bits::new();
   for &char in ascii_array {
      let bits = match char_bits_map.get(&char) {
         Some(x) => x,
         None => {
            continue;
         }
      };
      compressed_chars.extend(bits.iter());
   }

   let compressed_char_count = compressed_chars.len() as u32;
   let original_char_count = ascii_array.len() as u32;
   let tree_topology = encode_tree(&tree);
   let tree_topology_size = tree_topology.len() as u32;

   Code {
      compressed_char_count,
      original_char_count,
      tree_topology_size,
      tree_topology,
      compressed_chars,
   }
   .to_bits()
}

fn decode_tree(tree_topology: &Bits) -> Tree {
   fn to_u8(x: &BitSlice<u8, Msb0>) -> u8 {
      (x[0] as u8) << 7
         | (x[1] as u8) << 6
         | (x[2] as u8) << 5
         | (x[3] as u8) << 4
         | (x[4] as u8) << 3
         | (x[5] as u8) << 2
         | (x[6] as u8) << 1
         | x[7] as u8
   }

   let mut stack: VecDeque<Node> = VecDeque::new();
   let mut count: usize = 0;
   loop {
      if tree_topology[count] {
         count += 1;
         let char = to_u8(&tree_topology[count..count + 8]);
         count += 8;
         stack.push_back(Node::Leaf(Leaf { char }));
      } else {
         if stack.len() < 2 {
            break;
         }

         count += 1;
         let branch = Node::Branch(Branch {
            right: Box::new(stack.pop_back().unwrap()),
            left: Box::new(stack.pop_back().unwrap()),
         });
         stack.push_back(branch);
      }
   }
   Tree {
      root: Box::new(stack.pop_back().unwrap()),
   }
}

pub fn decode(bits: &Bits) -> String {
   let code = Code::from(bits);
   let tree = decode_tree(&code.tree_topology);
   let bits_char_map = generate_bits_char_map(&tree);
   let mut plane = String::new();
   let mut bits: Bits = BitVec::new();
   for bit in code.compressed_chars {
      bits.push(bit);
      match bits_char_map.get(&bits) {
         Some(x) => {
            plane.push(*x as char);
            bits.clear();
         }
         None => {
            continue;
         }
      }
   }
   plane
}
