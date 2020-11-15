/**
 * @param {number[]} nums
 * @param {number} x
 * @return {number}
 */
var minOperations = function(nums, x) {
  var current = 0;
  for (var num of nums) {
    current += num;
  } 
  
  var n = nums.length;
  var mini = Infinity;
  var left = 0;
  
  for (var right=0;right<n;right++) {
    current -= nums[right];
    while(current < x && left <= right) {
      current += nums[left];
      left += 1;
    }
    
    if (current === x) {
      mini = Math.min(mini, (n-1-right)+left);
    }
  }
  
  return mini != Infinity ? mini : -1;
};
