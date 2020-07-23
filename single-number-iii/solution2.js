/**
 * @param {number[]} nums
 * @return {number[]}
 */
var singleNumber = function(nums) {
  var xor = nums.reduce((acc, v) => acc ^ v);
  var rightbit = xor & ~(xor - 1);
  var ans = new Array(2);
  for (var i=0;i<nums.length;i++) {
    if (nums[i] & rightbit) {
      ans[0] ^= nums[i];
    } else {
      ans[1] ^= nums[i];
    }
  }
  return ans;
};
