/**
 * @param {number} low
 * @param {number} high
 * @return {number[]}
 */
var cache = [];

var sequentialDigits = function(low, high) {
  var lows = [...("" + low)];
  var highs = [...("" + high)];
  
  var lowlen = lows.length;
  var highlen = highs.length;

  if (cache.length === 0) {
    var arr = [[]];
    for (var i=1;i<10;i++) {
      arr[0].push(i);
    }
    for (var l=1;l<9;l++) {
      // console.log({l});
      if (arr[l] == null) {
        arr.push([]);
      }
      var ps = arr[l-1];
      for (var j=0;j<ps.length;j++) {
        // console.log({ps: ps[j]});
        var n = ps[j] % 10 + 1;
        if (n > 9) {
          break;
        }
        var next = ps[j]*10 + n;
        arr[l].push(next);
      }
    }
    cache = arr;
  }
  
  var result = cache.slice(lowlen-1, highlen).flat().filter((x) => x >= low && x <= high);  
  return result;
};
