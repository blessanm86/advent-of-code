import fs from "fs";

function getParsedFileContent(filename) {
  try {
    const data = fs.readFileSync(filename, "utf8");
    return data.split("\n").map((box) => box.split("x"));
  } catch (err) {
    console.error(err);
  }
}

function getBowLength(length, width, height) {
  const perimeters = [
    2 * length + 2 * width,
    2 * length + 2 * height,
    2 * width + 2 * height,
  ];

  const minPerimeter = Math.min.apply(null, perimeters);
  return minPerimeter + length * width * height;
}

console.assert(getBowLength(2, 3, 4) === 34);
console.assert(getBowLength(1, 1, 10) === 14);

//puzzle
const totalLength = getParsedFileContent("puzzle-input.txt")
  .map((dimensions) => getBowLength(...dimensions))
  .reduce((acc, next) => acc + next, 0);
console.log(totalLength); //5875601362
