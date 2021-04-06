/**
 * @param {number} n
 * @return {number}
 */
var minOperations = function(n) {
  if (n % 2 === 0) {
    return n * n / 4;    
  }
  return (n * n - 1) / 4;
};
