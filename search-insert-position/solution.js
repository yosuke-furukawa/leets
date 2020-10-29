/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var searchInsert = function(nums, target) {
  nums.unshift(-Infinity);
  nums.push(Infinity);
  var start = 0;
  var end = nums.length-1;
  var mid;
  var premid = mid;
  while (start < end) {
    mid = Math.ceil((start + end) / 2);
    var num = nums[mid];
    if (premid === mid) {
      break;
    }
    if (num > target) {
      end = mid;
    } else if (num < target) {
      start = mid;
    } else {
      break;
    }
    premid = mid;
  }
  return mid-1;
};
