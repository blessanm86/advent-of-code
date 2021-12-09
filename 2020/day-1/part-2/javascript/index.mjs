import fs from "fs";

function getParsedFileContent(filename) {
  try {
    const data = fs.readFileSync(filename, "utf8");
    return data.split("\n").map((num) => parseInt(num, 10));
  } catch (err) {
    console.error(err);
  }
}

function solve(expenses, count = 2, total, sum = 0) {
  if (count === 1) {
    return expenses.find((exp) => exp + sum === total);
  } else {
    for (let i = 0; i < expenses.length; i++) {
      const slice = expenses.slice(i + 1);
      const num = solve(slice, count - 1, total, expenses[i] + sum);
      if (num) {
        return [expenses[i], num].flat();
      }
    }
  }
}

const testResult = solve(getParsedFileContent("test-input.txt"), 3, 2020);
console.log(
  testResult,
  testResult.reduce((a, b) => a * b, 1)
); //979, 366, 675 = 241861950

const questionResult = solve(getParsedFileContent("input.txt"), 3, 2020);
console.log(
  questionResult,
  questionResult.reduce((a, b) => a * b, 1)
); // 928, 547, 545 = 276650720
