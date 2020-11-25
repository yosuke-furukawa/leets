/**
 * @param {number} K
 * @return {number}
 */
var smallestRepunitDivByK = function(K) {
  var remainder = 0;
  for (var length_N = 1; length_N <= K; length_N++) {
    remainder = (remainder * 10 + 1) % K;
    if (remainder == 0) {
      return length_N;
    }
  }
  return -1;
};
