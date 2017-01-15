extern crate sudoku;
use sudoku::sudoku_action::*;


use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::time::{SystemTime};

fn main() {
    let f = File::open("D:\\rustPro\\sudoku-rust\\config\\Input.json").unwrap();
    let mut br = BufReader::new(f);
    
    let mut s = String::new();
    br.read_to_string(&mut s).unwrap();
    
    let mut sdk = Sudoku::from_json_new(&s);

    let now = SystemTime::now();
    let mut rel_vec = Vec::new();
    for x in 0..1 {
        let mut tp = sdk.clone();
        tp.generate_sudoku(&mut rel_vec);
    }

    for r in rel_vec {
        println!("{}",r);
    }
    let du = now.elapsed().unwrap();
    println!("{:?}",du);
    //let encoded = json::encode(&sdk).unwrap();
    
    //println!("{}",sdk);
    //println!("{}",encoded);
}
