
/**
 * @param {number} p
 * @param {number} q
 * @return {number}
 */
var mirrorReflection = function(p, q) {
  if (p === q) {
    return 1;
  }
  var l = lcm(p, q);
  var w = l / q;
  var h = l / p;
  if (w % 2 === 0) {
    if (h % 2 === 0) {
      return 0;
    } else {
      return 2;
    }
  } else {
    if (h % 2 === 0) {
      return 0;
    } else {
      return 1;
    }
  }
};

function gcd(a, b) {
  if (b === 0){
    return a
  }
  return gcd(b, a % b)
}

function lcm(a, b) {
  var d = gcd(a, b);
  return a / d * b;
}
