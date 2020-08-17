/**
 * @param {number[]} nums
 * @return {number}
 */
var findMaxConsecutiveOnes = function(nums) {
  var max = 0;
  var count = 0;
  for (var n of nums) {
    if (n === 1) {
      max = Math.max(max, ++count);
    } else {
      count = 0;
    }
  }
  return max;
};
