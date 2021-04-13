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
let publicKey = 'df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074';
let isTx = false;
let lastHash = false;
let verboseDebug = true;

// Manage array of passwords
let fields = ["World"];

// Methods
/**
 * @param {Array} a : Array of solutions
 * @return {Boolean}
 */
function prove (a) {
  if (!Array.isArray(a)) {
    return false;
  } else if (!a.length) {
    return false;
  }
  // Generate hash
  let generatedProof = (isTx) ? h.g(JSON.stringify(a), (DEFAULT_OP_SIZE - 1)) : h.g(JSON.stringify(a), DEFAULT_DEPTH);
  // Is verified proof
  const v = (isTx) ? h.v(generatedProof, publicKey, DEFAULT_OP_SIZE, (DEFAULT_OP_SIZE - 1)) : h.v(generatedProof, publicKey, DEFAULT_SIZE, DEFAULT_DEPTH);
  lastHash = generatedProof;
  return v;
}

// Worker
function main () {
  // Basic prover
  let verified = prove(fields), msg;

  // Verbose debug
  if (verboseDebug) {
    let e = JSON.stringify(fields), size = (isTx) ? DEFAULT_OP_SIZE : DEFAULT_SIZE;
    let truthChain = [{depth: 0, private: e}];
    for (let i = 1; i < size; i++) {
      e = h.g(e, i);
      if (e) {
        truthChain.push({depth: i, proof: e});
      } else {
        console.warn('Error creating tree');
        return;
      }
      if (i == (size - 1)) {
        truthChain.push({depth: size, public: publicKey});
      }
    }
    console.log("Full hash chain:\n", truthChain);
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