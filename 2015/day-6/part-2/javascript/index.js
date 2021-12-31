const fs = require("fs");

function getParsedFileContent(filename) {
  try {
    return fs.readFileSync(filename, "utf8");
  } catch (err) {
    console.error(err);
  }
}

function parseInput(input) {
  return input
    .split("\n")
    .filter(Boolean)
    .map((i) => {
      let ins =
        /(?<operation>\D+)\s(?<startX>\d+),(?<startY>\d+)\D+(?<endX>\d+),(?<endY>\d+)/g.exec(
          i.trim()
        ).groups;

      return {
        operation: ins.operation,
        startX: Number(ins.startX),
        startY: Number(ins.startY),
        endX: Number(ins.endX),
        endY: Number(ins.endY),
      };
    });
}

function createMatrix(rows, columns, initialValue) {
  let matrix = {};

  for (i = 0; i < rows; i++) {
    for (j = 0; j < columns; j++) {
      matrix[`${i},${j}`] = initialValue;
    }
  }

  function get(row, column) {
    return matrix[`${row},${column}`];
  }

  function set(row, column, value) {
    matrix[`${row},${column}`] =
      typeof value === "function" ? value(get(row, column)) : value;
  }

  function setRange(rowStart, columnStart, rowEnd, columnEnd, value) {
    for (let i = rowStart; i <= rowEnd; i++) {
      for (let j = columnStart; j <= columnEnd; j++) {
        set(i, j, value);
      }
    }
  }

  function getValues() {
    return Object.values(matrix);
  }

  return {
    setRange,
    getValues,
  };
}

function solve(ins) {
  const matrix = createMatrix(1000, 1000, 0);

  ins.forEach((instruction) => {
    const { operation, startX, startY, endX, endY } = instruction;

    if (operation === "turn on") {
      matrix.setRange(startX, startY, endX, endY, (value) => value + 1);
    } else if (operation === "turn off") {
      matrix.setRange(startX, startY, endX, endY, (value) =>
        Math.max(value - 1, 0)
      );
    } else {
      matrix.setRange(startX, startY, endX, endY, (value) => value + 2);
    }
  });

  return matrix.getValues().reduce((acc, value) => acc + value, 0);
}

let testInput = `
    turn on 0,0 through 0,0
    toggle 0,0 through 999,999
`;

console.log(solve(parseInput(testInput))); // 2000001
console.log(solve(parseInput(getParsedFileContent("input.txt")))); // 14110788
