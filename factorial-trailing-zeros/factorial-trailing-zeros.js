/**
 * @param {number} n
 * @return {number}
 */
var trailingZeroes = function(n) {
  var s = 0;
  for (var x=1;5**x<=n;x++) {
    s += Math.floor(n / (5 ** x));
  } 
  return s;
};
