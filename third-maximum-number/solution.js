/**
 * @param {number[]} nums
 * @return {number}
 */
var thirdMax = function(nums) {
  nums.sort((a, b) => b-a);
  if (nums.length < 3) {
    return nums[0];
  }
  
  var set = new Set(nums);
  if ([...set][2] != null) {
    return [...set][2];
  } else {
    return [...set][0];
  }
};
