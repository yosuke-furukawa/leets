/**
 * @param {number} n
 * @return {string[]}
 */
const map = {
  "6": "9",
  "9": "6",
  "8": "8",
  "1": "1",
  "0": "0",
};

var memo = [["0", "1", "8"], ["00","11","69","88","96"]];

var findStrobogrammatic = function(n) {
  
  if (n === 1) {
    return memo[n-1];
  } else if (memo[n-1]) {
    return memo[n-1].filter((n) => n[0] !== "0");
  }
  
  for (var i=2;i<n;i++) {
    if (memo[i]) {
      continue;
    }
    var prev = memo[i-2];
    let res = [];
    for (var a of ["0", "1", "6", "8", "9"]) {
      var b = map[a];
      for (var p of prev) {
        res.push(`${a}${p}${b}`);
      }
    }
    memo.push(res);
  }
  return memo[memo.length-1].filter((n) => n[0] !== "0");
};
