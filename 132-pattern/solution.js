/**
 * @param {number[]} nums
 * @return {boolean}
 */
var find132pattern = function(nums) {
  if (nums.length < 3) {
    return false;
  }
  var min = [nums[0]];
  for (var i=1;i<nums.length;i++) {
    min.push(Math.min(min[i - 1], nums[i]));
  }
  for (var j = nums.length - 1, k = nums.length; j >= 0; j--) {
    if (nums[j] > min[j]) {
      while (k < nums.length && nums[k] <= min[j]) {
        k++;
      }
      
      if (k < nums.length && nums[k] < nums[j]) {
        return true;
      }
      nums[--k] = nums[j];
    }
  }
  return false;
};
