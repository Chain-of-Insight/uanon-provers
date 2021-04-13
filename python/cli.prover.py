#!/usr/bin/env python3

import sys, hashlib

def hashman(b):
    h = hashlib.blake2b(digest_size=32)
    h.update(b)
    return h.digest()

def generate_proof(message, depth):
    prefix = b'\x05\x01'
    len_bytes = (len(message)).to_bytes(4, byteorder='big')
    b = bytearray()
    b.extend(message.encode())
    proof = prefix + len_bytes + b
    for i in range(0, depth):
        proof = hashman(proof)
    return '0x' + proof.hex()

def main():
    if len(sys.argv) != 3:
        print("Usage: %s <message> <depth>" % (sys.argv[0]))
        sys.exit(1)
    print( generate_proof(sys.argv[1], int(sys.argv[2])) )

if __name__ == "__main__":
    main()