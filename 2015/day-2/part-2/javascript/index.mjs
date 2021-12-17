import fs from "fs";

function getParsedFileContent(filename) {
  try {
    const data = fs.readFileSync(filename, "utf8");
    return data.split("\n").map((box) => box.split("x"));
  } catch (err) {
    console.error(err);
  }
}

function getArea(length, width, height) {
  const topArea = length * width;
  const widthArea = width * height;
  const lengthArea = length * height;

  const minArea = Math.min(topArea, widthArea, lengthArea);
  return 2 * topArea + 2 * widthArea + 2 * lengthArea + minArea;
}

console.assert(getArea(2, 3, 4) === 58);
console.assert(getArea(1, 1, 10) === 43);

//puzzle
const totalArea = getParsedFileContent("puzzle-input.txt")
  .map((dimensions) => getArea(...dimensions))
  .reduce((acc, next) => acc + next, 0);
console.log(totalArea); //1598415
