/**
 * @param {number[]} arr
 * @param {number} k
 * @return {number}
 */
var findKthPositive = function(arr, k) {
  var num = 1;
  for (var i=0;i<k;) {
    var n = arr.length > 0 ? arr[0] : Infinity;
    if (num < n) {
      i++;
      num++;
    } else {
      arr.shift();
      num++;
    }
  }
  return num-1;
};
