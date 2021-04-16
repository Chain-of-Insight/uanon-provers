# uanon-provers

##### Implementations of the Uanon PoPMG crypto-proofs system in various programming languages. These hashers can be edited to verify you have the correct solution a puzzle listed by the [Project Uanon Puzzle Oracle](https://better-call.dev/mainnet/KT1VJsKdNFYueffX6xcfe6Gg9eJA6RUnFpYr/operations) on Tezos. Language implementations include examples of both the first Tutorial (https://uanon.observer/tutorial/1) and Tutorial ascension puzzles (https://uanon.observer/learn - accessible only if you solved Tutorials 1 - 5).

## Environment
For testing the Tutorial ascension scripts you'll need to add your proofs for Tutorials 1 - 5 into a `.env` file in the parent folder.
```
cp env.example .env
# Edit .env with your subproofs
```

## Node.js
```
cd js
npm install

# Prove Tutorial 1
npm run helloworld

# Prove Tutorial Ascension
npm run ascension
```

## Python3
```
cd python
pip install python-dotenv
pip install hashlib

# Prove Tutorial 1
python uanon.helloworld.py

# Prove Tutorial Ascension
python uanon.ascension.py

# Hash arbitrary string to an arbitrary depth
chmod +x cli.prover.py
./cli.prover.py "[\"World\"]" 2
# Returns string:
# > 0xdf69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074
```

## Rust
```
cd rust/

# Prove Tutorial 1
cd uanon-rs-helloworld/
cargo build
./target/debug/uanon-rs

# Prove Tutorial Ascension
cd ../uanon-rs-ascension/
cargo build
./target/debug/uanon-rs
```

## Go
```
# XXX TODO: add this
```
