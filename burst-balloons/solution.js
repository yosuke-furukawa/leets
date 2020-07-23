/**
 * @param {number[]} nums
 * @return {number}
 */

var maxCoins = function(nums) {
  var n = nums.length + 2;
  var newNums = [...nums];
  newNums.unshift(1);
  newNums.push(1);

  // cache the results of dp
  var memo = new Array(n);
  for (var i = 0;i<memo.length;i++) {
    memo[i] = new Array(n).fill(0);
  }
  return dp(memo, newNums, 0, n - 1);
};

function dp(memo, nums, left, right) {
  if (left + 1 === right) {
    return 0;
  }
  
  if (memo[left][right] > 0) {
    return memo[left][right];
  }
  
  var ans = 0;
  for (var i = left + 1; i < right; ++i) {
    ans = Math.max(ans, nums[left] * nums[i] * nums[right]
            + dp(memo, nums, left, i) + dp(memo, nums, i, right));
  }
  memo[left][right] = ans;
  return ans;
}
