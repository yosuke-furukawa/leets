/**
 * @param {number} K
 * @return {number}
 */
var smallestRepunitDivByK = function(K) {
  if (K % 2 === 0) {
    return -1;
  }
  
  if (K % 5 === 0) {
    return -1;
  }
  if (K < 5) {
    return K;
  }
  
  var N = 1n;
  K = BigInt(K);
  for (var i=2;i<10**5;i++) {
    N = N * 10n + 1n;
    if (N % K === 0n) {
      return i;
    }
  }
  return K;
};
