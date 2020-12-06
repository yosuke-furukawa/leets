/**
 * @param {number[]} nums
 * @return {number}
 */
var jump = function(nums) {
  var n = nums.length;
  if (n < 2) return 0;

  var maxPos = nums[0];
  var maxSteps = nums[0];
    
  var jumps = 1;
  for (var i = 1; i < n; ++i) {
    if (maxSteps < i) {
      ++jumps;
      maxSteps = maxPos;
    }
    maxPos = Math.max(maxPos, nums[i] + i);
  }
  return jumps;
};
