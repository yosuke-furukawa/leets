/**
 * @param {number} N
 * @param {number} K
 * @return {number}
 */
var kthGrammar = function(N, K) {
  if (K === 1) {
    return 0
  }
  if (K === 2) {
    return 1
  }
  
  const n = 2 ** (N-1);
  var val;
  if (K > n) {
    val = kthGrammar(N-1, K - n);
    return val === 0 ? 1 : 0;
  } else {
    val = kthGrammar(N-1, K);
    return val;
  }
};
