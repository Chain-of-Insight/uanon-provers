require('dotenv').config({path:'../.env'});

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

// Init
const H = require('./hasher');

const h = {
  g: H.generateProofAsString, 
  v: H.verifyProof
};

const DEFAULT_SIZE = 2;
const DEFAULT_OP_SIZE = 1001;
const DEFAULT_DEPTH = 1;

// Configuration
let publicKey = 'afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce';
let isTx = true;
let lastHash = false;
let verboseDebug = true;

// Replace these with your own proofs derived from the output of the `log` command from console mode after solving Tutorials 1 to 5.
let subproofT1 = process.env.T1,
    subproofT2 = process.env.T2,
    subproofT3 = process.env.T3,
    subproofT4 = process.env.T4,
    subproofT5 = process.env.T5;
// Manage array of passwords
let fields = [
  // Previous proofs of season
  subproofT1,
  subproofT2,
  subproofT3,
  subproofT4,
  subproofT5,
  // Current puzzle solutions
  'Well done player',
  'We wait for light but behold darkness',
  'I wish you a world free of demons and full of light',
  'U'
];

// Methods
/**
 * @param {Array} a : Array of solutions
 * @param {Boolean} verbose : Enable verbose debug output
 * @return {Boolean}
 */
function prove (a, verbose = false) {
  if (!Array.isArray(a)) {
    return false;
  } else if (!a.length) {
    return false;
  }
  // Generate hash
  let generatedProof = (isTx) ? h.g(JSON.stringify(a), (DEFAULT_OP_SIZE - 1), verbose) : h.g(JSON.stringify(a), DEFAULT_DEPTH, verbose);
  // Is verified proof
  const v = (isTx) ? h.v(generatedProof, publicKey, DEFAULT_OP_SIZE, (DEFAULT_OP_SIZE - 1), verbose) : h.v(generatedProof, publicKey, DEFAULT_SIZE, DEFAULT_DEPTH, verbose);
  lastHash = generatedProof;
  return v;
}

// Worker
function main () {
  // Basic prover
  let verified, msg;

  // Verbose debug
  if (verboseDebug) {
    verified = prove(fields, true);
  } else {
    verified = prove(fields);
  }

  // Regular debug
  if (verified) {
    msg = 'Proof could not be verified';
    console.error(msg, {
      proofHash: lastHash.substring(2),
      publicKey: publicKey
    });
  } else {
    msg = 'Proof verification succeeded';
    console.log(msg, {
      proofHash: lastHash.substring(2),
      publicKey: publicKey
    });
  }
}

main();