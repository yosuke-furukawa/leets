/**
 * @param {number[]} nums
 * @param {number} S
 * @return {number}
 */
var findTargetSumWays = function(nums, S) {
  var stack = [0];
  for (var i=0;i<nums.length;i++) {
    var tmp = [];
    while (stack.length > 0) {
      tmp.push(stack.pop());
    }
    var n = nums[i];
    while (tmp.length > 0) {
      var pre = tmp.pop();
      stack.push(pre + n);
      stack.push(pre - n);
    }
  }
  return stack.filter((s) => s === S).length;
};
