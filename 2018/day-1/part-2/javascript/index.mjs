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
  let tempResults = {};
  let sum = 0;
  let index = 0;
  const end = freqs.length - 1;

  while (true) {
    const newFreq = sum + freqs[index];
    let result = tempResults[newFreq];

    if (result) {
      return newFreq;
    } else {
      tempResults[newFreq] = 1;
    }

    sum += freqs[index];

    index = index === end ? 0 : index + 1;
  }
}

console.log(solve(getParsedFileContent("input.txt"))); //71892
