/**
 * @param {number[]} nums
 * @return {number}
 */
var findUnsortedSubarray = function(nums) {
  var sorted = [...nums].sort((a, b) => a-b);
  var start = -1;
  var end = -1;
  for (var i=0;i<nums.length;i++) {
    if (start < 0 && nums[i] !== sorted[i]) {
      start = i;
      break;
    }
  }
  
  for (var i=nums.length-1;i>=0;i--) {
    if (end < 0 && nums[i] !== sorted[i]) {
      end = i;
      break;
    }
  }
  return end === start ? 0 : end - start + 1;
};
