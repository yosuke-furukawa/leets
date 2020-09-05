/**
 * @param {number} num
 * @return {boolean}
 */
var isPerfectSquare = function(num) {
  if (num < 2) return true;

  var x0 = num;
  var x1 = (x0 + num / x0) / 2;
  while (Math.abs(x0 - x1) >= 1) {
    x0 = x1;
    x1 = (x0 + num / x0) / 2;
  }

  // console.log(x1);
  var sqrt = Math.floor(x1);
  return sqrt ** 2 === num;
};
