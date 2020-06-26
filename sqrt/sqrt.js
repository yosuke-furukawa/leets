/**
 * @param {number} x
 * @return {number}
 */
var mySqrt = function(x) {
  if (x < 2) return x;

  var x0 = x;
  var x1 = (x0 + x / x0) / 2.0;
  while (Math.abs(x0 - x1) >= 1) {
    x0 = x1;
    x1 = (x0 + x / x0) / 2.0;
  }

  return Math.floor(x1);
};
