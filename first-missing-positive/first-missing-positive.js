/**
 * @param {number[]} nums
 * @return {number}
 */
var firstMissingPositive = function(nums) {
  nums = nums.filter((num) => num > 0);
  for (var i=0;i<nums.length;i++) {
    var x = Math.abs(nums[i]);
    if (x-1 < nums.length && nums[x-1] > 0) {
      nums[x-1] = -nums[x-1];
    }
  }
  
  for (var i=0;i<nums.length;i++) {
    if (nums[i] > 0) {
      return i+1
    }
  }
  
  return nums.length + 1;
};
