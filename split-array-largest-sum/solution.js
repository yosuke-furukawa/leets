/**
 * @param {number[]} nums
 * @param {number} m
 * @return {number}
 */
var splitArray = function(nums, m) {
  var dp = new Array(nums.length).fill(0);
  var sum = new Array(nums.length).fill(0);
  
  sum[0] = nums[0];
  for (var i=1;i<nums.length;i++) {
    sum[i] = sum[i-1] + nums[i];
  }
  dp[0] = sum[sum.length-1];
  
  for (var i=1;i<nums.length;i++) {
    dp[i] = sum[nums.length-1] - sum[i-1];
  }
  
  for (var i=1;i<m;i++) {
    for (var start=0;start<nums.length-i;start++) {
      for (var end=start+1;end<=nums.length-i;end++) {
        dp[start] = Math.min(dp[start], Math.max((
          start===0 ? sum[end-1] : sum[end-1] - sum[start-1]
        ),dp[end]));
      }
    }
  }
  
  return dp[0];
};
