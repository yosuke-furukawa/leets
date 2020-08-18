/**
 * @param {number[]} nums
 * @return {number[]}
 */
var findDisappearedNumbers = function(nums) {
  for (var i=0;i<nums.length;i++) {
    var newIndex = Math.abs(nums[i]) - 1;
    if (nums[newIndex] > 0) {
      nums[newIndex] *= -1;
    }
  }
  
  var result = [];
  for (var i=0;i<nums.length;i++) {
    if (nums[i] > 0) {
      result.push(i+1);
    }
  }
  return result;
};
