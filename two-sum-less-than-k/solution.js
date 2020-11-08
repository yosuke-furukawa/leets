/**
 * @param {number[]} A
 * @param {number} K
 * @return {number}
 */
var twoSumLessThanK = function(A, K) {
  var sum = -1;
  var minDiff = Infinity;
  for (var i=0;i<A.length;i++) {
    for (var j=i+1;j<A.length;j++) {
      var s = A[i] + A[j];
      if (K - s > 0 && minDiff > K - s) {
        minDiff = K - s;
        sum = s;
      }
    }
  }
  return sum;
};
