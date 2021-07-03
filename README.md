# uanon-provers

##### Implementations of the Uanon PoPMG crypto-proofs system in various programming languages. These hashers can be edited to verify you have the correct solution to puzzles listed in the [Project Uanon Puzzle Oracle](https://better-call.dev/mainnet/KT1VJsKdNFYueffX6xcfe6Gg9eJA6RUnFpYr/operations) on Tezos. Language implementations include examples of both the first Tutorial (https://uanon.observer/tutorial/1) and Tutorial ascension puzzles (https://uanon.observer/learn - accessible only if you solved Tutorials 1 - 5).

## Environment
For testing the Tutorial ascension scripts you'll need to add your proofs for Tutorials 1 - 5 into a `.env` file in the parent folder.
```
cp env.example .env
# Edit .env with your subproofs
```

## Node.js
```
cd js/
npm install

# Prove Tutorial 1
npm run helloworld

# Prove Tutorial Ascension
npm run ascension
```

## Python3
```
cd python/
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
cd go/
ln -s ../.env ./

# Prove Tutorial 1
go build uanon.helloworld.go
./uanon.helloworld

# Prove Tutorial Ascension
go build uanon.ascension.go
./uanon.ascension
```

---

### Perfomance Stats:

_Benchmarked on i7-7700 @ 3.6GHz_

#### Node.js
```
$ time npm run helloworld
# ...
> real    0m0.219s
> user    0m0.209s
> sys     0m0.046s

$ time npm run ascension
# ...
> real    0m0.299s
> user    0m0.304s
> sys     0m0.024s

# Removing npm call wrapper increases speed
$ time node uanon.helloworld.js
# ...
> real    0m0.041s
> user    0m0.033s
> sys     0m0.016s

$ time node uanon.ascension.js
# ...
> real    0m0.126s
> user    0m0.111s
> sys     0m0.023s
```

#### Python3
```
$ time python3 uanon.helloworld.py
# ...
> real    0m0.029s
> user    0m0.028s
> sys     0m0.000s

$ time python3 uanon.ascension.py
# ...
> real    0m0.054s
> user    0m0.054s
> sys     0m0.000s 
```

#### Rust
```
$ time ./uanon-rs-helloworld/target/debug/uanon-rs
# ...
> real    0m0.001s
> user    0m0.002s
> sys     0m0.000s

$ time ./uanon-rs-ascension/target/debug/uanon-rs
# ...
> real    0m0.035s
> user    0m0.009s
> sys     0m0.026s
```

#### Go
```
$ time ./uanon.helloworld
# ...
> real    0m0.004s
> user    0m0.004s
> sys     0m0.000s

$ time ./uanon.ascension
# ...
real    0m0.011s
user    0m0.012s
sys     0m0.000s
```
