const fs = require("fs");

function getParsedFileContent(filename) {
  try {
    return fs.readFileSync(filename, "utf8").split("\n");
  } catch (err) {
    console.error(err);
  }
}

function isNice(str) {
  //match atleast 3 vowels
  if ((str.match(/[aeiou]/gi)?.length || 0) < 3) {
    return false;
  }

  //match 2 repeating characters
  if (!str.match(/(\w)\1/gi)) {
    return false;
  }

  //should not match the patterns
  if (str.match(/ab|cd|pq|xy/gi)) {
    return false;
  }

  return true;
}

console.assert(isNice("ugknbfddgicrmopn") === true);
console.assert(isNice("aaa") === true);
console.assert(isNice("jchzalrnumimnmhp") === false);
console.assert(isNice("haegwjzuvuyypxyu") === false);
console.assert(isNice("dvszwmarrgswjxmb") === false);

let strs = getParsedFileContent("input.txt");
console.log(strs.filter(isNice).length);
