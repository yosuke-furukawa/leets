/**
 * @param {number[]} nums
 * @return {number}
 */
var arrayPairSum = function(nums) {
  nums = nums.sort((a, b) => a-b);
  var j;
  var sum = 0;
  for (var i=0;i<nums.length;i+=2) {
    var j = i+1;
    var min = Math.min(nums[i], nums[j]);
    sum += min;
  }
  return sum;
};
