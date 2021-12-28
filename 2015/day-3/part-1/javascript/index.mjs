import fs from "fs";

function getParsedFileContent(filename) {
  try {
    return fs.readFileSync(filename, "utf8");
  } catch (err) {
    console.error(err);
  }
}

function solve(ins) {
  let houses = {};
  let startX = 0;
  let startY = 0;
  let luckyHouseCount = 0;

  houses["0,0"] = 1;

  ins.split("").forEach((x) => {
    switch (x) {
      case ">": {
        startX += 1;
        break;
      }
      case "<": {
        startX -= 1;
        break;
      }
      case "^": {
        startY += 1;
        break;
      }
      case "v": {
        startY -= 1;
        break;
      }
    }

    let pos = `${startX},${startY}`;

    if (houses[pos]) {
      luckyHouseCount += 1;
      houses[pos] += 1;
    } else {
      houses[pos] = 1;
    }
  });

  return Object.keys(houses).length;
}

console.assert(solve(">") === 2);
console.assert(solve("^>v<") === 4);
console.assert(solve("^v^v^v^v^v") === 2);

console.log(solve(getParsedFileContent("input.txt"))); //2572
