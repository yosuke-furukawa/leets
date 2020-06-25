/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
var uniquePaths = function(m, n) {
  var w = m - 1;
  var h = n - 1;
  
  // (w+h)C(h)
  var result = 1;
  var count = h;
  var i = w+h;
  while(count>0) {
    result *= i;
    result /= count;
    count--;
    i--;
  }
  return result;
};
