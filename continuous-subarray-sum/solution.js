/**
 * @param {number[]} nums
 * @param {number} k
 * @return {boolean}
 */
var checkSubarraySum = function(nums, k) {
  var prev = null;
  for (var i=0;i<nums.length;i++) {
    var curr = new Array(nums.length).fill(0);
    if (prev != null) {
      for (var j=i+1;j<nums.length;j++) {
        curr[j] = prev[j]-nums[i-1];
        // console.log(curr[j], i);
        if (k === 0 && curr[j] === 0) {
          return true;
        }
        if (curr[j] % k === 0) {
          return true;
        }
      }
      prev = curr;
    } else {
      for (var j=i+1;j<nums.length;j++) {
        curr[j] = [...nums.slice(i, j+1)].reduce((a, b) => a+b);
        if (k === 0 && curr[j] === 0) {
          return true;
        }
        if (curr[j] % k === 0) {
          return true;
        }
      }
      prev = curr;
    }
  }
  return false;
};
