'''
Example 1 uses the first Tutorial. This puzzle does not include a Tezos and transaction and is not an ascension. You can access the puzzle at https://uanon.observer/tutorial/1. 

Title: Hello World
Description: The site says: {payload}
Public Key: df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074
Operation: opYLmo1SAtwSWzQEMfmAmFRBucqAFZu42591XwcmgyVJrEQsUKN
Payload: Hello
Format: 1 word, uppercase first letter
'''

import hashlib
import json

# Configuration
t1_public_key = 'df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074'
pad_size = 4
is_tezos_transaction = False
debug = True
debug_verbose = False

if is_tezos_transaction is True:
    hashing_rounds = 1001
else:
    hashing_rounds = 2

# Manage array of passwords
fields = ["World"]
fields = json.dumps(fields, separators=(',',':'))
len_bytes = (len(fields)).to_bytes(pad_size, byteorder='big')
fields = fields.encode()

# Data prefix
prefix = b'\x05\x01'
raw_secret = prefix + len_bytes + fields

# Rounds settings init
iterations = list(range(hashing_rounds))
last_hash = False
last_hash_b = None
attempts = []

# Hashing rounds
for i in iterations:
    if last_hash is False:
        attempts.append(raw_secret)
        hash_i = hashlib.blake2b(raw_secret, digest_size=32)
        if debug is True:
            print('Raw Secret:', raw_secret)
    else:
        hash_i = hashlib.blake2b(last_hash_b, digest_size=32)

    last_hash = hash_i.hexdigest()
    last_hash_b = hash_i.digest()
    attempts.append(last_hash)

    if debug is True:
        print('Iteration ' + str(i) + ':', last_hash)

# What is True?
assert last_hash == t1_public_key, 'Final hash does not match public key'

# The opposite of False
print('\nFinal hash (' + last_hash + ') and Public Key (' + t1_public_key + ') are equivalent\n')