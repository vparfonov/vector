// Copyright 2019 TiKV Project Authors. Licensed under MIT or Apache-2.0.

use std::env::args;

// CRC-64/NVME polynomial
//
// NVM Express® NVM Command Set Specification (Revision 1.0d, December 2023)
//
// https://nvmexpress.org/wp-content/uploads/NVM-Express-NVM-Command-Set-Specification-1.0d-2023.12.28-Ratified.pdf
const POLY: u64 = 0xAD93D23594C93659;

// usage:
//
//  ./build_table 0    # generate TABLE_0
//  ./build_table 1    # generate TABLE_1
//
// etc.

fn long_div_step(m: u64) -> u64 {
    m << 1 ^ if m >> 63 != 0 { POLY } else { 0 }
}

fn main() {
    let table_id = args().nth(1).unwrap().parse::<u32>().unwrap();
    println!("static TABLE_{}: [u64; 256] = [", table_id);
    let count = table_id * 8 + 8;
    for i in 0..=255u8 {
        let byte = i.reverse_bits();
        let mut value = u64::from(byte) << 56;
        for _ in 0..count {
            value = long_div_step(value);
        }
        println!("    {:#018x},", value.reverse_bits());
    }
    println!("];");
}
