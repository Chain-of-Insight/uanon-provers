/**
 * Example 1 uses the first Tutorial. This puzzle does not include a Tezos and transaction and is not an ascension. You can access the puzzle at https://uanon.observer/tutorial/1. 
 *
 * Title: Hello World
 * Description: The site says: {payload}
 * Public Key: df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074
 * Operation: opYLmo1SAtwSWzQEMfmAmFRBucqAFZu42591XwcmgyVJrEQsUKN
 * Payload: Hello
 * Format: 1 word, uppercase first letter
 */

extern crate hex;
extern crate array_init;
extern crate blake2_rfc;

use std::str;
use blake2_rfc::blake2b::{Blake2b};

static PUBLIC_KEY: &str = "df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074";

fn main() {
    // Configure script
    let iterations: u8 = 3; // Default chain size
    println!("Chain Size: {}", iterations);

    // Configure password args.
    let prefix = b"\x05\x01";
    let fieldstr = b"[\"World\"]";
    let prefix_s = str::from_utf8(prefix).unwrap();
    let fieldstr_s = str::from_utf8(fieldstr).unwrap();
    
    // Set padding
    let _usize = fieldstr.len();
    let len_bytes = _usize.to_be_bytes();
    let padded_bytes = &len_bytes[4.._usize-1];
    let padded_bytes_s = str::from_utf8(padded_bytes).unwrap();

    // Hash routine
    let mut hasher = Blake2b::new(32);
    hasher.update(prefix);
    hasher.update(padded_bytes);
    hasher.update(fieldstr);

    // Finalize
    let res = hasher.finalize();
    let out_b = res.as_bytes();
    let hashes: [&[u8]; 2] = [out_b, out_b];

    // Output, depth 0
    let out = hex::encode(out_b);
    println!("Raw Secret: {:?}{:?}{:?}", prefix_s, padded_bytes_s, fieldstr_s);
    println!("Depth 1: {:?}", out);

    let mut iter = hashes[..].windows(2);
    while let Some([prev, _next]) = iter.next() {
        // Chained hash routine
        let mut hasher = Blake2b::new(32);
        hasher.update(prev);
        
        // Finalize
        let res = hasher.finalize();
        let out_b = res.as_bytes();
        let out = hex::encode(out_b);
        println!("Depth 2: {:?}", out);
        
        // Validate
        assert_eq!(out, PUBLIC_KEY);
        println!("Final hash ({:?}) and Public Key ({:?}) are equivalent", out, PUBLIC_KEY);
    }
}