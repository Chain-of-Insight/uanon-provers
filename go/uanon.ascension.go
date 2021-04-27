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

 package main

 import (
     "os"
     "fmt"
     "testing"
     "encoding/hex"
     "encoding/json"
     "github.com/joho/godotenv"
     "golang.org/x/crypto/blake2b"
     "github.com/stretchr/testify/assert"
 )
 
 // Configurable
 const PUBLIC_KEY = "afbda72bc5ca82bc61d800fcc8fdfa4f059d95e58879795863b34525ded88fce"
 const PWD_ARRAY_SIZE = 9
 const iterations = 1001
 
 func hasher(data []byte) []byte {
    res := blake2b.Sum256(data)
    return res[:]
 }
 
 func dataLength(val int) string {
    x := fmt.Sprintf("%x", val)
    for len(x) < 8 {
        x = fmt.Sprintf("0%s", x)
    }
    return x
 }
 
 func genProof(msg string, depth int) []byte {
    proof, _ := hex.DecodeString(fmt.Sprintf("0501%s%s", dataLength(len(msg)), hex.EncodeToString([]byte(msg))))
    for i := 0; i < depth; i++ {
        proof = hasher(proof)
        hexed := hex.EncodeToString(proof)
        s := fmt.Sprintf("Depth %d: %s", (i+1), hexed)
        fmt.Println(s)
    }
    return proof
 }
 
 func AssertEquals(t *testing.T, res string) {
    assert := assert.New(t)
    equality := assert.Equal(res, PUBLIC_KEY)
    if equality {
        s := fmt.Sprintf("Final hash %s and Public Key %s are equivalent", res, PUBLIC_KEY)
        fmt.Println(s)   
    } else {
        fmt.Println("Proof verification failed")
    }
 }
 
 func main() {
    // Init
    err := godotenv.Load(".env")
    if err != nil {
        panic("Error loading environment")
    }

    // Assemble
    var passwords [PWD_ARRAY_SIZE]string
    passwords[0] = os.Getenv("T1")
    passwords[1] = os.Getenv("T2")
    passwords[2] = os.Getenv("T3")
    passwords[3] = os.Getenv("T4")
    passwords[4] = os.Getenv("T5")
    passwords[5] = "Well done player"
    passwords[6] = "We wait for light but behold darkness"
    passwords[7] = "I wish you a world free of demons and full of light"
    passwords[8] = "U"
    json, _ := json.Marshal(passwords)

    // Generate
    s := fmt.Sprintf("Raw Secret: %s", string(json))
    fmt.Println(s)
    res := genProof(string(json), iterations)
    hexed := hex.EncodeToString(res)
    
    // Validate
    t := &testing.T{}
    AssertEquals(t, hexed)
 }