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

package main

import (
    "fmt"
    "testing"
    "encoding/hex"
    "encoding/json"
    "golang.org/x/crypto/blake2b"
    "github.com/stretchr/testify/assert"
)

// Configurable
const PUBLIC_KEY = "df69b9d584c7594c819796d31b8c9b174a3c2f45f3a1e9f3443ce4831584c074"
const PWD_ARRAY_SIZE = 1
const iterations = 2

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
    var passwords [PWD_ARRAY_SIZE]string
    passwords[0] = "World"
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