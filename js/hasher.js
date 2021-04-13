const blake2b = require('blake2b');

const hash = function (b) {
  return blake2b(32).update(b).digest();
};

const deephash = function (b, depth, verbose = false) {
  for (let i = 0; i < depth; i++) {
    b = hash(b);
    if (verbose) {
      console.log({depth: (i + 1), proof: Buffer.from(b).toString('hex')});
    }
  }
  return b;
};

const generateProof = function (message, depth, verbose = false) {
  const input = Buffer.from(message);
  const prefix = Buffer.from("0501", 'hex');
  const len_bytes = Buffer.from(message.length.toString(16).padStart(8, '0'), 'hex');
  let proof = Buffer.concat([prefix, len_bytes, input], prefix.length + len_bytes.length + input.length);
  return deephash(proof, depth, verbose);
};

const generateProofAsString = function(message, depth, verbose = false) {
  let proofAsString = '0x' + Buffer.from(generateProof(message, depth, verbose)).toString('hex');
  return proofAsString;
};

const verifyProof = function (proof, verification, depth, atdepth, verbose = false) {
  depth = Math.abs(depth - atdepth);
  proof = deephash(Buffer.from(proof, 'hex'), depth);
  let hexProof = Buffer.from(proof).toString('hex');
  if (verbose) {
    console.log({depth: Number(atdepth + 1), public: verification});
  }
  return hexProof == verification;
}

module.exports = {
  generateProofAsString: generateProofAsString,
  verifyProof: verifyProof
};