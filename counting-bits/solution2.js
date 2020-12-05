/**
 * @param {number} num
 * @return {number[]}
 */
var countBits = function(num) {
  var dp = new Array(num+1).fill(0);
  var n = 1;
  for (var i=1;i<num+1;i++) {
    dp[i] = dp[i & (i - 1)] + 1;
  }
  return dp;
};
