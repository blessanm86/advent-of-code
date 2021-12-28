const md5 = require("md5");

function solve(input) {
  let counter = 0;

  while (true) {
    const hash = md5(`${input}${counter}`);
    const shouldStop = (hash.match(/^0+/)?.[0]?.length || 0) >= 6;

    if (shouldStop) {
      return counter;
    } else {
      counter++;
    }
  }
}

console.assert(solve("iwrupvqb") === 9958218);
