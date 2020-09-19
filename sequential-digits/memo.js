/**
 * @param {number} low
 * @param {number} high
 * @return {number[]}
 */
var sequentialDigits = function(low, high) {
  var lows = [...("" + low)];
  var highs = [...("" + high)];
  
  var lowlen = lows.length;
  var highlen = highs.length;
  var maxdigit = parseInt(lows[0]);
  var result = [];
  var latest = 0;
  var len = lowlen;
  var prev = [];
  var next = [];
  while (latest < high) {
    console.log({latest, len, result, maxdigit});
    if (prev.length === 0) {
      var n = 0;
      if (10 - maxdigit < len) {
        len++;
        maxdigit = 1;
        prev = [...next];
        continue;
      }
      if (10 - maxdigit >= len) {
        for (var m=maxdigit;m<maxdigit+len;m++) {
          // console.log({ n });
          n *= 10;
          n += m;     
        }
      
        if (n > high) {
          break;
        }
      
        if (low < n) {
          result.push(n);
          next.push(n);
          latest = n;
        } else {
          len++;
          maxdigit = 1;
          prev = [...next];
          continue;
        }
      }
      maxdigit++;
    } else {
      
    }
  }
  return result;
};
