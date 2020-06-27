/**
 * @param {number} numerator
 * @param {number} denominator
 * @return {string}
 */
var fractionToDecimal = function(numerator, denominator) {
  var num = Math.abs(numerator);
  var answer = [];
  var modmap = new Map();
  var sign = Math.sign(numerator) * Math.sign(denominator);
  denominator = Math.abs(denominator);
  while(true) {
    var ans = Math.floor(num / denominator);
    answer.push(ans);
    var mod = num % denominator;
    if (mod === 0) {
      break;
    }
    if (modmap.has(mod)) {
      var prev = modmap.get(mod);
      answer.splice(prev, 0, "(");
      answer.push(")");
      break;
    }
    modmap.set(mod, answer.length);
    num = mod * 10;
  }
  if (answer.length > 1) {
    answer.splice(1, 0, ".");
  }
  if (sign < 0) {
    answer.unshift("-");
  }
  return answer.join("");
};
