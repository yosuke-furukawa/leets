/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canPartition = function(nums) {
  var n = nums.length;
  var sum = nums.reduce((acc,curr) => acc+curr);
  if (sum % 2 !== 0) {
    return false;
  }
  sum = sum / 2;

  var dp = new Array(nums.length).fill(false).map(() => new Array(sum + 1).fill(false));
  for (let i = 0; i < nums.length; i++) {
    dp[i][0] = true;
  }
  
  for (let s = 1; s <= sum; s++) {
    dp[0][s] = nums[0] === s;
  }
  
  for (let i = 1; i < nums.length; i++) {
    for (let s = 1; s <= sum; s++) {
      if (dp[i - 1][s]) {
        dp[i][s] = dp[i - 1][s];
      } else if (s >= nums[i]) {
        dp[i][s] = dp[i - 1][s - nums[i]];
      }
    }
  }

  return dp[n - 1][sum];
};
