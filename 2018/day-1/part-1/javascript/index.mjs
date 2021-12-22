import fs from "fs";

function getParsedFileContent(filename) {
  try {
    const data = fs.readFileSync(filename, "utf8");
    return data.split("\n").map((num) => parseInt(num, 10));
  } catch (err) {
    console.error(err);
  }
}

function solve(freqs) {
  return freqs.reduce((acc, mass) => acc + mass, 0);
}

console.log(solve(getParsedFileContent("input.txt"))); //516
