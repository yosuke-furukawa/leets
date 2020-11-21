/**
 * @param {string[]} digits
 * @param {number} n
 * @return {number}
 */
var atMostNGivenDigitSet = function(digits, n) {
  var N = digits.length;
  var str = "" + n;
  var count = 0;
  var minusone = 0;
  var carry = 0;
  var prejs = [];
  for (var i=str.length-1;i>=0;i--) {
    var s = +str[i] + carry + "";
    carry = 0;
    var p = str.length - i - 1;
    var j=0;
    for (;j<digits.length;j++) {
      if (digits[j] === s) {
        break;
      } else if (+digits[j] > +s) {
        break;
      }
    }
    
    if (digits[j] !== s) {
      minusone = 1;
    }
    if (+digits[j] > +s) {
      for (var k=0;k<prejs.length;k++) {
        var prej = prejs[k];
        if (prej > 0) {
          count = count - Math.pow(N, k) * (prej+1);
          count = count + Math.pow(N, k);
          prejs[k] = 0;
        }
      }
      
    }
    
    if (j === digits.length) {
      j = 0;
      for (var k=0;k<prejs.length;k++) {
        var prej = prejs[k];
        if (prej > 0) {
          count = count - Math.pow(N, k) * (prej+1);
          count = count + Math.pow(N, k);
          prejs[k] = 0;
        }
      }
      carry = 1;
    }

    // console.log({ s, j, minusone, d: digits[j], carry, count, prejs, p })
    count += Math.pow(N, p) * (j+1);
    prejs.push(j);
  }
  if (carry === 1) {
    for (var k=0;k<prejs.length;k++) {
      var prej = prejs[k];
      if (prej > 0) {
        count = count - Math.pow(N, k) * (prej+1);
        count = count + Math.pow(N, k);
        prejs[k] = 0;
      }
    }
    count += Math.pow(N, p+1);
  }
  return count - minusone;
};
