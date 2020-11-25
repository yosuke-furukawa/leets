/**
 * @param {number} K
 * @return {number}
 */
var smallestRepunitDivByK = function(K) {
  if(!(K%2) || !(K%5)) return -1;
  var remainder = 1;
  var lengthN = 1;
  var seen = new Set();
  
  var N = 1;
  while (remainder % K !== 0) {
    N = remainder * 10 + 1;
    remainder = N % K;
    lengthN += 1;
    
    if (seen.has(remainder)) {
      return -1;
    } else {
      seen.add(remainder);
    }
  }
  return lengthN;
};
