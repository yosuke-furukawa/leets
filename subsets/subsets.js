/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsets = function(nums) {
  var result = [];
  var k = 0;
  var backtrack = function(first, curr, nums) {
    if (curr.length === k) {
      result.push([...curr]);
    }
    for (var i = first; i < nums.length; i++) {
      curr.push(nums[i]);
      backtrack(i + 1, curr, nums);
      curr.pop();
    }
  }
  for (k=0;k<=nums.length;k++) {
    backtrack(0, [], nums);
  }
  return result;
};
