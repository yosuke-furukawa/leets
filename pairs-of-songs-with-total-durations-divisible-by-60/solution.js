/**
 * @param {number[]} time
 * @return {number}
 */
var numPairsDivisibleBy60 = function(time) {
  var remainders = new Array(60).fill(0);
  var count = 0;
  for (var t of time) {
    if (t%60 === 0) {
      count += remainders[0];
    } else {
      count += remainders[60 - t % 60];
    }
    
    remainders[t%60]++;
  }
  return count;
};
