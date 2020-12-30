/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var threeSumSmaller = function(nums, target) {  
  var sorted = nums.sort((a, b) => a-b);
  var sum = 0;
  for (var i=0;i<nums.length-2;i++) {
    sum += twoSumSmaller(nums, i+1, target - nums[i]);
  }
  return sum;
};

function twoSumSmaller(nums, startIndex, target) {
  var sum = 0;
  var left = startIndex;
  var right = nums.length - 1;
  while (left < right) {
    if (nums[left] + nums[right] < target) {
      sum += right - left;
      left++;
    } else {
      right--;
    }
  }
  return sum;
}
