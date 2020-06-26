/**
 * @param {number} x
 * @param {number} n
 * @return {number}
 */
var myPow = function(x, n) {
  if (x === 1 || n === 0) {
    return 1;
  }
  if (x === -1) {
    return n%2 === 0 ? 1 : -1;
  }
  var sign = Math.sign(n);
  var absn = Math.abs(n);
  var num = 1;
  var X = x;
  for (var i=absn;i>0;i=Math.floor(i/2)) {
    if (i%2 === 1) {
      num = num * X;
    }
    X = X * X;
  }
  if (sign < 0) {
    num = 1/num;
  }
  return num;
};
