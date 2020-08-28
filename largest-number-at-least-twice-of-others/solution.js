/**
 * @param {number[]} nums
 * @return {number}
 */
var dominantIndex = function(nums) {
  var max = 0;
  var secondMax = 0;
  var maxIndex = 0;
  for (var i=0;i<nums.length;i++) {
    var n = nums[i];
    if (n > max) {
      secondMax = max;
      max = n;
      maxIndex = i;
    } else if (n > secondMax) {
      secondMax = n;
    }
  }
  return secondMax * 2 <= max ? maxIndex : -1;
};
