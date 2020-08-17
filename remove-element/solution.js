/**
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
var removeElement = function(nums, val) {
  for (var i =0; i<nums.length;i++) {
    var n = nums[i];
    if (n === val) {
      nums.splice(i, 1);
      i--;
    }
  }
  return nums.length;
};
