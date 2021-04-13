'''
Ascensions are special end of season puzzles. They require you submit all previous proofs of the season, in consecutive order, as well as the passwords of the current puzzle. If a puzzle has rewards but isn't the final puzzle of that season, you won't need to submit all of the sub proofs in addition to the current passwords, but you will need to increase the rounds of hashing. You can access this ascension example at https://uanon.observer/learn, but only if you're holding the correct sub-proofs.

Title: Through darkness and the ages, softly
Description: We ascend
Public Key: afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce
Operation: oorfEK3FTyx3ha5FPY8vmpEQ86z1xQemVMXGd5BkNbqRPJTXHpq
Format: Separate the sentences and add spaces between words. Cipher provides the case. No punctuation.
Hint: Solution 4 is a single character, if you're confused get your faqs straight
'''

import os
import dotenv
import hashlib
import json

# Configuration
asc_public_key = 'afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce'
pad_size = 4
is_tezos_transaction = True
debug = True
debug_verbose = False

dotenv.load_dotenv()

if is_tezos_transaction is True:
    hashing_rounds = 1001
else:
    hashing_rounds = 2

'''
Replace these with your own proofs derived from the output of the `log` command from console mode after solving Tutorials 1 to 5.
'''
# Sub-proofs
subproof_tutorial_1 = os.environ.get('T1')
subproof_tutorial_2 = os.environ.get('T2')
subproof_tutorial_3 = os.environ.get('T3')
subproof_tutorial_4 = os.environ.get('T4')
subproof_tutorial_5 = os.environ.get('T5')

# Manage array of passwords
fields = [
    # Previous proofs of season
    subproof_tutorial_1,
    subproof_tutorial_2,
    subproof_tutorial_3,
    subproof_tutorial_4,
    subproof_tutorial_5,
    # Current puzzle solutions
    'Well done player',
    'We wait for light but behold darkness',
    'I wish you a world free of demons and full of light',
    'U'
]
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
        print('Raw Secret:', raw_secret)
        hash_i = hashlib.blake2b(raw_secret, digest_size=32).digest()
    else:
        hash_i = hashlib.blake2b(last_hash_b, digest_size=32).digest()

    last_hash = hash_i.hex()
    last_hash_b = hash_i
    attempts.append(last_hash)
    print('Iteration ' + str(i+1) + ':', last_hash)

if debug_verbose is True:
    print('\nSolution chain:\n')
    print(attempts)

# What is True?
assert last_hash == asc_public_key, 'Final hash does not match public key'

# The opposite of False
print('\nFinal hash (' + last_hash + ') and Public Key (' + asc_public_key + ') are equivalent\n')