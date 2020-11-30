/**
 * @param {number[]} nums
 * @param {number} k
 * @return {boolean}
 */
var canPartitionKSubsets = function(nums, k) {
  var N = nums.length;
  nums = nums.sort((a, b) => a-b);

  var sum = nums.reduce((a, b) => a + b);
  var target = sum / k;
  if (sum % k > 0 || nums[N-1] > target) {
    return false;
  }
  
  var dp = new Array(1 << N).fill(false);
  dp[0] = true;
  var total = new Array(1 << N).fill(0);
  
  for (var state=0;state<(1 << N); state++) {
    if (!dp[state]) continue;
    for (var i=0;i<N;i++) {
      var future = state | (1 << i);
      if (state !== future && !dp[future]) {
        if (nums[i] <= target - (total[state] % target)) {
          dp[future] = true;
          total[future] = total[state] + nums[i];
        } else {
          break;
        }
      }
    }
  }
  return dp[(1 << N) - 1];
};
