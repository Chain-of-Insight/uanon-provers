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
extern crate json;
extern crate dotenv;

use std::str;
use recur_fn::{recur_fn, RecurFn};
use blake2_rfc::blake2b::{Blake2b, Blake2bResult};

static PUBLIC_KEY: &str = "df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074";

struct Hash {
    n: u32, 
    res: Blake2bResult,
}

fn main() {
    // Configure script
    let iterations: u32 = 2; // Default chain size;
    println!("Chain Size: {}", iterations);

    // Configure password args.
    let prefix = b"\x05\x01";
    let fieldstr = b"[\"World\"]";
    let fieldstr_s = str::from_utf8(fieldstr).unwrap();
    
    // Set padding
    let _usize = fieldstr.len();
    let len_bytes = _usize.to_be_bytes();
    let padded_bytes = &len_bytes[4..(len_bytes.len())];

    // Hash routine
    let mut hasher = Blake2b::new(32);
    hasher.update(prefix);
    hasher.update(padded_bytes);
    hasher.update(fieldstr);

    // Recursive hash routine
    let recur_hasher = recur_fn(|recur_hasher, mut h: Hash| {
        if h.n >= iterations {
            h
        } else {
            // Output
            let out_b = h.res.as_bytes();
            let out = hex::encode(out_b);
            println!("Depth {:?}: {:?}", h.n, out);
            
            // Instance
            let mut blake = Blake2b::new(32);
            blake.update(h.res.as_bytes());
            
            // Finalize
            let res = blake.finalize();
            h.res = res.clone();
            h.n += 1;
            
            // Return
            recur_hasher(h)
        }
    });

    println!("Raw Secret: {:?}", fieldstr_s);

    let start = Hash {
        n: 1,
        res: hasher.finalize(),
    };
    let recb = recur_hasher.call(start);
    let final_out = hex::encode(recb.res.as_bytes());

    assert_eq!(final_out, PUBLIC_KEY);
    println!("Final hash ({:?}) and Public Key ({:?}) are equivalent", final_out, PUBLIC_KEY);
}