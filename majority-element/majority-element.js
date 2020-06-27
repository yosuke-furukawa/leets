/**
 * @param {number[]} nums
 * @return {number}
 */
var majorityElement = function(nums) {
  var candidate;
  var count = 0;
  for (var num of nums) {
    if (count === 0) {
      candidate = num;
      count++;
    } else if (candidate === num) {
      count++;
    } else {
      count--;
    }
  }
  return candidate;
};
