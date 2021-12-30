const fs = require("fs");

function getParsedFileContent(filename) {
  try {
    return fs.readFileSync(filename, "utf8").split("\n");
  } catch (err) {
    console.error(err);
  }
}

function isNice(str) {
  //match pair characters
  if (!/(\w\w).*\1/i.test(str)) {
    return false;
  }

  //match repeating character
  if (!/(\w)\w\1/i.test(str)) {
    return false;
  }

  return true;
}

console.assert(isNice("qjhvhtzxzqqjkmpb") === true);
console.assert(isNice("xxyxx") === true);
console.assert(isNice("uurcxstgmygtbstg") === false);
console.assert(isNice("ieodomkazucvgmuy") === false);

let strs = getParsedFileContent("input.txt");
console.log(strs.filter(isNice).length);
