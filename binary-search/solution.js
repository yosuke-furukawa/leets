/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function(nums, target) {
  var left = 0;
  var right = nums.length-1;
  var middle = Math.floor((right + left) /2);
  
  while (left <= right) {
    var m = nums[middle];
    if (m === target) {
      return middle;
    }
    if (m < target) {
      left = middle+1;
    } else {
      right = middle-1;
    }
    middle = Math.floor((right + left) /2);
  }
  return -1;
};
