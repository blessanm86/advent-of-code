import fs from "fs";

function getParsedFileContent(filename) {
  try {
    const data = fs.readFileSync(filename, "utf8");
    return data.split("\n").map((num) => parseInt(num, 10));
  } catch (err) {
    console.error(err);
  }
}

function getFuel(fuel) {
  const neededFuel = Math.floor(fuel / 3) - 2;

  if (neededFuel > 0) {
    return neededFuel + getFuel(neededFuel);
  }

  return 0;
}

function solve(masses) {
  return masses.map(getFuel).reduce((acc, mass) => acc + mass, 0);
}

console.log(solve(getParsedFileContent("input.txt"))); //5035632
