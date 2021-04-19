/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var combinationSum4 = function(nums, target) {
  var dp = new Array(target + 1).fill(0);
  dp[0] = 1;
  
  for (var combSum = 1; combSum < target + 1; combSum++) {
    for (var num of nums) {
      if (combSum - num >= 0) {
        dp[combSum] += dp[combSum - num];
      }
    }
  }
  return dp[target];
};
