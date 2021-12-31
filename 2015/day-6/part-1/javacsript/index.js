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

function createMatrix(rows, columns, initialValue = false) {
  let matrix = Array(rows)
    .fill()
    .map((row) => Array(columns).fill(initialValue));

  function get(row, col) {
    return matrix[row][col];
  }

  function set(row, col, value) {
    matrix[row][col] = value;
  }

  function setRange(startRow, starCol, endRow, endCol, value) {
    for (let i = startRow; i <= endRow; i++) {
      for (let j = starCol; j <= endCol; j++) {
        set(i, j, typeof value === "function" ? value(i, j, get(i, j)) : value);
      }
    }
  }

  function filter(filterFn = Boolean) {
    return matrix.flatMap((x) => x).filter(filterFn);
  }

  return {
    setRange,
    filter,
  };
}

function solve(ins) {
  const matrix = createMatrix(1000, 1000, false);

  ins.forEach((instruction) => {
    const { operation, startX, startY, endX, endY } = instruction;

    if (operation === "turn on") {
      matrix.setRange(startX, startY, endX, endY, true);
    } else if (operation === "turn off") {
      matrix.setRange(startX, startY, endX, endY, false);
    } else {
      matrix.setRange(startX, startY, endX, endY, (row, col, value) => {
        return !value;
      });
    }
  });

  return matrix.filter().length;
}

let testInput = `
    turn on 0,0 through 999,999
    toggle 0,0 through 999,0
    turn off 499,499 through 500,500
`;

console.log(solve(parseInput(testInput))); // 998996
console.log(solve(parseInput(getParsedFileContent("input.txt")))); // 377891
