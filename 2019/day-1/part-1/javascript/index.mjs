import fs from "fs";

function getParsedFileContent(filename) {
  try {
    const data = fs.readFileSync(filename, "utf8");
    return data.split("\n").map((num) => parseInt(num, 10));
  } catch (err) {
    console.error(err);
  }
}

function solve(masses) {
  return masses
    .map((mass) => Math.floor(mass / 3) - 2)
    .reduce((acc, mass) => acc + mass, 0);
}

console.log(solve(getParsedFileContent("input.txt"))); //3358992
