/**
 * @param {number} n
 * @return {number}
 */
var nextGreaterElement = function(n) {
  var nums = [...String(n)];
  var max = nums[nums.length - 1];
  for (var i=nums.length-2;i>=0;i--) {
    if (nums[i] < max) {
      var sliced = nums.slice(i).sort((a, b) => a-b);
      var index = sliced.findIndex((n) => n > nums[i]);
      var n2 = sliced.splice(index, 1);
      nums[i] = n2;
      for (var s = 0;s<sliced.length;s++) {
        nums[i+s+1] = sliced[s];
      }
      break;
    }
    max = nums[i];
  }
  var ans = Number(nums.join(""));
  if (ans === n) {
    return -1;
  }
  if (ans > 2**31 - 1) {
    return -1;
  }
  return ans;
};
