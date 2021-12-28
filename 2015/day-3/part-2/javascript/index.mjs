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
  let startSantaX = 0;
  let startSantaY = 0;
  let startRoboX = 0;
  let startRoboY = 0;
  let luckyHouseCount = 0;

  houses["0,0"] = 1;

  ins.split("").forEach((x, index) => {
    let pos;

    if (index % 2 === 0) {
      switch (x) {
        case ">": {
          startRoboX += 1;
          break;
        }
        case "<": {
          startRoboX -= 1;
          break;
        }
        case "^": {
          startRoboY += 1;
          break;
        }
        case "v": {
          startRoboY -= 1;
          break;
        }
      }

      pos = `${startRoboX},${startRoboY}`;
    } else {
      switch (x) {
        case ">": {
          startSantaX += 1;
          break;
        }
        case "<": {
          startSantaX -= 1;
          break;
        }
        case "^": {
          startSantaY += 1;
          break;
        }
        case "v": {
          startSantaY -= 1;
          break;
        }
      }

      pos = `${startSantaX},${startSantaY}`;
    }

    if (houses[pos]) {
      luckyHouseCount += 1;
      houses[pos] += 1;
    } else {
      houses[pos] = 1;
    }
  });

  return Object.keys(houses).length;
}

console.assert(solve("^v") === 3);
console.assert(solve("^>v<") === 3);
console.assert(solve("^v^v^v^v^v") === 11);

console.log(solve(getParsedFileContent("input.txt"))); //2631
