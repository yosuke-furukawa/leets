/**
 * @param {number[]} arr
 * @return {number}
 */
var maxNumberOfApples = function(arr) {
  var sorted = arr.sort((a, b) => a-b);
  var sum = 0;
  var i;
  for (i=0;i<sorted.length;i++) {
    var s = sum + sorted[i];
    if (s > 5000) {
      break;
    }
    sum = s;
  }
  return i;
};
