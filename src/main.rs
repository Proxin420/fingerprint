use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::str::{self, from_utf8};
use std::mem;


fn cipher(block: Vec<u8>, base_block: &Vec<u8>, map: &Vec<u8>) -> Vec<u8> {
    let mut out_block: Vec<u8> = Vec::new();
    let mut count = 0;
    println!("block: {:?}", block);
    println!("base_block {:?}", base_block);
    for i in block {
        out_block.push(i^base_block[count]);
        let s_box = map[usize::from(out_block[count])]; 
        mem::replace(&mut out_block[count], s_box); 
        count = count + 1;
    }
    println!("out_block: {:?}", out_block);
    return out_block;
}


fn main() {
    let map: Vec<u8> = [
    86,148,229,146,17,127,199,9,2,100,
    134,204,175,186,151,196,164,104,
    98,145,103,173,57,76,47,105,3,
    138,121,242,188,150,187,247,
    93,69,172,22,64,198,77,248,87,
    132,149,167,216,159,142,128,99,189,246,
    135,208,42,210,20,162,90,192,4,
    233,197,91,58,92,6,81,133,96,217,206,
    30,219,117,36,26,170,227,16,
    19,203,230,125,115,45,89,165,0,
    27,11,110,1,29,136,56,243,68,232,
    34,205,106,54,166,143,48,71,38,
    84,254,234,253,67,31,75,163,60,82,
    255,147,240,33,12,191,108,97,5,
    119,44,241,59,213,160,171,215,83,
    153,201,15,177,222,235,211,14,
    221,183,154,174,41,7,179,24,72,
    79,194,250,50,236,66,52,202,157,
    155,112,35,8,80,141,10,40,49,46,
    118,207,223,55,61,239,224,181,130,
    32,120,74,113,212,21,228,200,152,
    182,176,116,51,129,73,65,156,111,
    169,158,95,244,249,70,124,185,220,
    78,190,168,37,214,94,218,226,193,114,
    53,18,13,231,25,109,144,28,39,101,
    225,131,126,43,251,62,23,195,209,
    245,123,137,102,184,161,238,85,178,
    180,107,140,88,237,252,122,139,63,].to_vec();
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);
    let mut block: Vec<u8> = Vec::new(); // This variable Holds a vector of the current block(16 bytes)
    let mut count = 1;
    let mut block_count = 0;
    let mut base_block: Vec<u8> = Vec::new();
    
    // Loop trough file and give each block(16 bytes) to the cipher function
    for i in reader.bytes() {
        let i = i.unwrap();
        //let byte = from_utf8(i).unwrap();
        block.push(i);
        if count == 16 {
            if block_count == 0 {
                base_block = block.clone();
                //println!("block: {:?}", block);
            } else {
                base_block = cipher(block, &base_block, &map);
            }
            count = 1;
            block_count = block_count + 1;
            block = Vec::new();
        } else {
            count = count + 1;
        }
    }
    // Add padding if the last block(16 bytes) is not 16 bytes long
    if count != 1 {
        while count != 17 {
            block.push(0);
            count = count + 1;
        }
        block_count = block_count + 1;
        base_block = cipher(block, &base_block, &map);
        block = Vec::new();
    }
    
    println!("Amount of blocks: {}", block_count);
    println!("FingerPrint: {:?}", base_block);
}
