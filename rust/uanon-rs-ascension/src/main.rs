/**
 * Ascensions are special end of season puzzles. They require you submit all previous proofs of the season, in consecutive order, as well as the passwords of the current puzzle. If a puzzle has rewards but isn't the final puzzle of that season, you won't need to submit all of the sub proofs in addition to the current passwords, but you will need to increase the rounds of hashing. You can access this ascension example at https://uanon.observer/learn, but only if you're holding the correct sub-proofs.
 *
 * Title: Through darkness and the ages, softly
 * Description: We ascend
 * Public Key: afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce
 * Operation: oorfEK3FTyx3ha5FPY8vmpEQ86z1xQemVMXGd5BkNbqRPJTXHpq
 * Format: Separate the sentences and add spaces between words. Cipher provides the case. No punctuation.
 * Hint: Solution 4 is a single character, if you're confused get your faqs straight
 */

extern crate hex;
extern crate json;
extern crate dotenv;
extern crate blake2_rfc;

use std::str;
use recur_fn::{recur_fn, RecurFn};
use blake2_rfc::blake2b::{Blake2b, Blake2bResult};
 
static PUBLIC_KEY: &str = "afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce";

struct Hash {
    n: u32, 
    res: Blake2bResult,
}

fn main() {
    // Initialize environment variables (sub-proofs)
    dotenv::dotenv().ok();

    // Configure script
    let iterations: u32 = 1001; // Default chain size
    println!("Chain Size: {}", iterations);

    // Configure password args.
    let prefix = b"\x05\x01";

    // Sub-proofs
    let subproof_tutorial_1 = dotenv::var("T1").unwrap();
    let subproof_tutorial_2 = dotenv::var("T2").unwrap();
    let subproof_tutorial_3 = dotenv::var("T3").unwrap();
    let subproof_tutorial_4 = dotenv::var("T4").unwrap();
    let subproof_tutorial_5 = dotenv::var("T5").unwrap();

    // Assembled raw array
    let fields = vec![
        subproof_tutorial_1.to_string(),
        subproof_tutorial_2.to_string(),
        subproof_tutorial_3.to_string(),
        subproof_tutorial_4.to_string(),
        subproof_tutorial_5.to_string(),
        "Well done player".to_string(),
        "We wait for light but behold darkness".to_string(),
        "I wish you a world free of demons and full of light".to_string(),
        "U".to_string()
    ];
    
    // As JSON bytes
    let fieldsjson = json::stringify(fields);
    let fieldstr = fieldsjson.as_bytes();
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

    // Finalize
    let res = hasher.finalize();
    let out_b = res.as_bytes();

    // Output, depth 0, 1
    let out = hex::encode(out_b);
    println!("Raw Secret: {:?}", fieldstr_s);
    println!("Depth 1: {:?}", out);

    // Recusive hash routine
    let recur_hasher = recur_fn(|recur_hasher, mut h: Hash| {
        if h.n < 1 {
            h
        } else if h.n >= iterations {
            h
        } else {
            // Instance
            let mut blake = Blake2b::new(32);
            blake.update(h.res.as_bytes());
            // Finalize
            let res = blake.finalize();
            h.res = res.clone();
            h.n += 1;
            // Output
            let out_b = h.res.as_bytes();
            let out = hex::encode(out_b);
            println!("Depth {:?}: {:?}", h.n, out);
            // Return
            recur_hasher(h)
        }
    });
    let start = Hash {
        n: 1,
        res: res.clone(),
    };
    let recb = recur_hasher.call(start);
    let final_out = hex::encode(recb.res.as_bytes());

    assert_eq!(final_out, PUBLIC_KEY);
    println!("Final hash ({:?}) and Public Key ({:?}) are equivalent", final_out, PUBLIC_KEY);
}