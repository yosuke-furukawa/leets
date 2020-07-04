/**
 * @param {number} n
 * @return {number}
 */

var nthUglyNumber = function(n) {
  var two = 2;
  var three = 3;
  var five = 5;
  var i2 = 0;
  var i3 = 0;
  var i5 = 0;
  var result = [1];
  for (var i=1;i<n;) {
    var min = Math.min(two, three, five);
    if (min === two) {
      two = result[i2] * 2;
      i2++;
    } else if (min === three) {
      three = result[i3] * 3;
      i3++;
    } else {
      five = result[i5] * 5;
      i5++;
    }
    if (result[result.length-1] === min) {
      continue;
    }
    result.push(min);
    i++;
  }
  return result[n-1];
};
