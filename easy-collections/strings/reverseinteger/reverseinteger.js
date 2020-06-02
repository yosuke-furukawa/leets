var reverse = function(x) {
  var result = 0;
  var sign = x < 0 ? -1 : 1;
  var s = sign * x;
  var max = (2**31) - 1;
  var min = (2**31) * -1;
  while (s > 0) {
    var r = s % 10;
    s = Number.parseInt(s / 10);
    result = 10 * result + r;
    if (sign * result >= max) {
      return 0;
    }
    if (sign * result <= min) {
      return 0;
    }
  }
  return sign * result;
};
