/**
 * @param {number[]} nums
 * @return {number}
 */
var pivotIndex = function(nums) {
  var left = [];
  var lsum = 0;
  for (var i=0;i<nums.length;i++) {
    lsum += nums[i];
    left.push(lsum);
  }
  
  var right = [];
  var rsum = 0;
  for (var i=nums.length-1;i>=0;i--) {
    rsum += nums[i];
    right.unshift(rsum);
  }
  
  
  for (var i=0;i<left.length;i++) {
    if (left[i] === right[i]) {
      return i;
    }
  }
  return -1;
};
