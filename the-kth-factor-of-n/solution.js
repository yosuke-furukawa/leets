/**
 * @param {number} n
 * @param {number} k
 * @return {number}
 */
var kthFactor = function(n, k) {
  var result = [];
  for (var i=1;i<=n;i++) {
    if (n%i === 0) {
      result.push(i);
    }
    if (result.length === k) {
      return result[k-1];
    }
  }
  return -1;
};
