#![feature(is_sorted)]
mod btree;
use std::fs;

use bitvec::prelude::*;
use btree::{Value, Node};

use crate::btree::pretty_print_tree;

fn main() {
    println!("Hello, world!");
    let mut values = vec![
        "friend","test","Something","amazing","test","test","Something","Something","another","thing","thing","thing"
    ].iter().map(|s| Value::Value(s.to_string())).collect();
    let tree = Node::make_tree(&mut values);
    pretty_print_tree(&tree,0,false);
    let encodings = tree.generate_encodings();
    for (k,v) in encodings.iter() {
        println!("{:?}: {}",k,v);
    }
    let mut encoder: BitVec<u8, Lsb0> = BitVec::new();
    for val in values.iter() {
        encoder.append(&mut encodings.get(val).unwrap().clone());
    }
    println!("encoded: {:X}",encoder);
    let bytes: Vec<u8> = encoder.clone().into_vec();
    let _ = fs::write("test.bin", bytes.clone());
    let b2b: BitVec<u8, Lsb0> = BitVec::from_vec(bytes);
}
