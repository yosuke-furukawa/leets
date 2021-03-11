/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
var coinChange = function(coins, amount) {
  var dp = [];
  for (var i=0;i<=coins.length+1;i++) {
    dp[i] = new Array(amount+1).fill(0);
  }
  for (var j=1;j<amount+1;j++) {
    dp[0][j] = Infinity;
  }
  for (var i=0;i<dp.length-1;i++) {
    for (var j=0;j<dp[i].length;j++) {
      if (j >= coins[i]) {
        dp[i+1][j] = Math.min(dp[i][j], dp[i+1][j-coins[i]] + 1);
      } else {
        dp[i+1][j] = dp[i][j];
      }
    }
  }
  if (dp[coins.length][amount] === Infinity) {
    return -1;
  }
  return dp[coins.length][amount];
};
