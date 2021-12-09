import fs from "fs";

function getParsedFileContent(filename) {
  try {
    const data = fs.readFileSync(filename, "utf8");
    return data.split("\n").map((num) => parseInt(num, 10));
  } catch (err) {
    console.error(err);
  }
}

function solve(expenses) {
  for (let i = 0; i < expenses.length; i++) {
    for (let j = 0; j < expenses.length; j++) {
      if (expenses[i] + expenses[j] === 2020) {
        return expenses[i] * expenses[j];
      }
    }
  }
}

console.log(solve(getParsedFileContent("test-input.txt"))); //1721 * 299 = 514579
console.log(solve(getParsedFileContent("input.txt"))); //618 * 1402 = 866436
