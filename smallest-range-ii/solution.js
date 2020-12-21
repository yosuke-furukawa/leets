/**
 * @param {number[]} A
 * @param {number} K
 * @return {number}
 */
var smallestRangeII = function(A, K) {
  var sorted = A.sort((a, b) => a-b);
  for (var i=0;i<sorted.length;i++) {
    sorted[i] -= K;
  }
  var current = Math.abs(sorted[sorted.length - 1] - sorted[0]);
  for (var i=0;i<sorted.length;i++) {
    sorted[i] += 2 * K;
    var min = Math.min(...sorted);
    var max = Math.max(...sorted);
    var c = Math.min(current, Math.abs(max - min));
    if (current > c) {
      current = c;
    }
  }
  return current;
};
