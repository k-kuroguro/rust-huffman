use bit_vec::BitVec;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Weight = u64;

//ASCI(8-bits) only

struct Tree {
   root: Box<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Leaf {
   weight: Weight,
   char: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Branch {
   weight: Weight,
   right: Box<Node>,
   left: Box<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
enum Node {
   Branch(Branch),
   Leaf(Leaf),
}

impl Node {
   fn weight(&self) -> Weight {
      match self {
         Node::Branch(x) => x.weight,
         Node::Leaf(x) => x.weight,
      }
   }
}

impl Ord for Node {
   fn cmp(&self, other: &Self) -> Ordering {
      self.weight().cmp(&other.weight())
   }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HeapData {
   weight: Reverse<Weight>,
   node: Box<Node>,
}

const PLANE: &[u8] = b"This sentence contains three a's, three c's, two d's, twenty-six e's, five f's, three g's, eight h's, thirteen i's, two l's, sixteen n's, nine o's, six r's, twenty-seven s's, twenty-two t's, two u's, five v's, eight w's, four x's, five y's, and only one z.";

fn evaluate_weight(plane: &[u8]) -> Vec<Leaf> {
   let mut evaluated_chars: HashSet<u8> = HashSet::new();
   let mut leafs: Vec<Leaf> = Vec::new();
   for char in plane {
      if evaluated_chars.contains(&char) {
         continue;
      }
      leafs.push(Leaf {
         weight: plane.iter().filter(|&x| x == char).count() as Weight,
         char: *char,
      });
      evaluated_chars.insert(*char);
   }
   leafs
}

fn generate_tree(leafs: Vec<Leaf>) -> Option<Tree> {
   let mut heap: BinaryHeap<HeapData> = BinaryHeap::from(
      leafs
         .iter()
         .map(|x| HeapData {
            weight: Reverse(x.weight),
            node: Box::new(Node::Leaf(x.clone())),
         })
         .collect::<Vec<_>>(),
   );

   loop {
      if heap.len() <= 1 {
         break;
      }

      // Heap length is always checked before pop.
      let left = match heap.pop() {
         Some(x) => x,
         None => unreachable!(),
      };
      let right = match heap.pop() {
         Some(x) => x,
         None => unreachable!(),
      };

      let branch = Node::Branch(Branch {
         weight: left.weight.0.saturating_add(right.weight.0),
         left: left.node,
         right: right.node,
      });
      heap.push(HeapData {
         weight: Reverse(branch.weight().clone()),
         node: Box::new(branch),
      });
   }

   match heap.pop() {
      Some(x) => Some(Tree { root: x.node }),
      None => None,
   }
}

type CharBitsMap = HashMap<u8, BitVec>;

//TODO: 文字が1種類の場合
fn search_tree_children(node: Box<Node>, char_bits_map: &mut CharBitsMap, bits: &mut BitVec) {
   match *node {
      Node::Branch(x) => {
         let mut left_bits = bits.clone();
         left_bits.push(false);
         search_tree_children(x.left, char_bits_map, &mut left_bits);
         let mut right_bits = bits.clone();
         right_bits.push(true);
         search_tree_children(x.right, char_bits_map, &mut right_bits);
      }
      Node::Leaf(x) => {
         char_bits_map.insert(x.char, bits.clone());
      }
   }
}

fn generate_char_bits_map(tree: &Tree) -> CharBitsMap {
   let mut char_bits_map: CharBitsMap = HashMap::new();
   let mut bits = BitVec::new();
   search_tree_children(tree.root.clone(), &mut char_bits_map, &mut bits);
   char_bits_map
}

fn encode_tree(tree: &Tree) -> BitVec {
   let mut result = BitVec::new();
   fn search_post_order(node: Box<Node>, bits: &mut BitVec) {
      match *node {
         Node::Branch(x) => {
            search_post_order(x.left, bits);
            search_post_order(x.right, bits);
            bits.push(false);
         }
         Node::Leaf(x) => {
            bits.push(true);
            bits.append(&mut BitVec::from_bytes(&[x.char]));
         }
      }
   }
   search_post_order(tree.root.clone(), &mut result);
   result.push(false);
   result
}

fn encode(plane: &[u8], tree: &Tree) -> BitVec {
   let char_bits_map = generate_char_bits_map(tree);
   let mut header = encode_tree(tree);
   let mut result = BitVec::new();
   result.append(&mut header);
   for char in plane {
      let bits = match char_bits_map.get(&char) {
         Some(x) => x,
         None => {
            continue;
         }
      };
      result.extend(bits.iter());
   }
   result
}

fn main() {}
