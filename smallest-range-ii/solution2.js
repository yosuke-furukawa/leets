/**
 * @param {number[]} A
 * @param {number} K
 * @return {number}
 */
var smallestRangeII = function(A, K) {
  var sorted = A.sort((a, b) => a-b);
  var current = Math.abs(sorted[sorted.length - 1] - sorted[0]);
  for (var i=0;i<sorted.length-1;i++) {
    var min = Math.min(sorted[0] + K, sorted[i+1] - K);
    var max = Math.max(sorted[sorted.length - 1] - K, sorted[i] + K);
    current = Math.min(current, max - min);
  }
  return current;
};
