/**
 * @param {number} n
 * @return {number}
 */
var arrangeCoins = function(n) {
  // x(x-1)/2 = n
  // 2*n = x(x-1);
  // x^2 - x - 2*n =0
  // x = (1 + sqrt(1+8n))/2
  if (n<0) {
    return n;
  }
  return Math.floor((1 + Math.sqrt(1+8*n))/2) - 1;
};
