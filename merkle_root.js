const hash256 = (input) => {
  const h1 = createHash("sha256").update(Buffer.from(input, "hex")).digest();
  return createHash("sha256").update(h1).digest("hex");
};

txids = [
  "8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87",
  "fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4",
  "6359f0868171b1d194cbee1af2f16ea598ae8fad666d9b012c8ed2b79a236ec4",
  "e9a66845e05d5abc0ad04ec80f774a7e585c6e8db975962d069a522137b80c1d",
];

// reverse the txids
let level = txids.map((txid) =>
  Buffer.from(txid, "hex").reverse().toString("hex")
);

while (level.length > 1) {
  const nextLevel = [];

  for (let i = 0; i < level.length; i += 2) {
    let pairHash;
    if (i + 1 === level.length) {
      // In case of an odd number of elements, duplicate the last one
      pairHash = hash256(level[i] + level[i]);
    } else {
      pairHash = hash256(level[i] + level[i + 1]);
    }
    nextLevel.push(pairHash);
  }

  level = nextLevel;
}
console.log(level[0]);