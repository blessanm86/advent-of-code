const md5 = require("md5");

function solve(input) {
  let counter = 0;

  while (true) {
    const hash = md5(`${input}${counter}`);
    const shouldStop = (hash.match(/^0+/)?.[0]?.length || 0) >= 5;

    if (shouldStop) {
      return counter;
    } else {
      counter++;
    }
  }
}

console.assert(solve("abcdef") === 609043);
console.assert(solve("pqrstuv") === 1048970);
console.assert(solve("iwrupvqb") === 346386);
console.log(solve("iwrupvqb")); //346386
