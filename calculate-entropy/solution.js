/**
 * @param {number[]} input
 * @return {number}
 */
var calculateEntropy = function(input) {
  var countMap = new Map();
  for (var i=0;i<input.length;i++) {
    var n = input[i];
    if (countMap.has(n)) {
      countMap.set(n, countMap.get(n) + 1);
    } else {
      countMap.set(n, 1);
    }
  }
  var set = new Set(input);
  var result = 0;
  for (var n of set) {
    var p = countMap.get(n) / input.length;
    result += -1 * p * Math.log2(p);
  }
  return result;
};
