/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var searchRange = function(nums, target) {
  var low = 0;
  var high = nums.length - 1;
  
  while(low <= high) {
    var mid = low + Math.floor((high - low) /2);
    if (nums[mid] === target) {
      low = mid;
      high = mid;
      while(nums[low] === target || nums[high] === target) {
        if (nums[low] === target) {
          low--;
        }
        if (nums[high] === target) {
          high++;
        }
      }
      return [low+1, high-1];
    }
    if (nums[mid] < target) {
      low = mid + 1;
      
    } else {
      high = mid - 1;
    }
  }
  return [-1, -1];
};
