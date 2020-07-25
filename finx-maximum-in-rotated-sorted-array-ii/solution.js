/**
 * @param {number[]} nums
 * @return {number}
 */
var findMin = function(nums) {
  if (nums.length == 1) {
    return nums[0];
  }

  var min = nums[0];
  for (var i=1;i<nums.length;i++) {
    if (min > nums[i]) {
      min = nums[i];
      break;
    }
  }
  return min;
};
