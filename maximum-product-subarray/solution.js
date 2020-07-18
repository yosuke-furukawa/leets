/**
 * @param {number[]} nums
 * @return {number}
 */
var maxProduct = function(nums) {
  if (nums.length == 0) return 0;

  var max = nums[0];
  var min = nums[0];
  var result = max;

  for (var i = 1; i < nums.length; i++) {
    var curr = nums[i];
    var tempMax = Math.max(curr, Math.max(max * curr, min * curr));
    min = Math.min(curr, Math.min(max * curr, min * curr));

    max = tempMax;

    result = Math.max(max, result);
  }

  return result;
};
