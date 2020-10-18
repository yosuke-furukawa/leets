/**
 * @param {number} k
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(k, prices) {
  var n = prices.length;
        
  if (k > parseInt(n/2)) {
    var res = 0;
    for (var i = 1; i < n; i ++) {
      res += Math.max(0,prices[i]-prices[i-1]);
    }
    return res;
  }
        
  var dp = new Array(k+1);
  for (var x=0;x<dp.length;x++) {
    dp[x] = new Array(n).fill(0);
  }
  for (var i=1;i<=k;i++) {
    var profit = 0 - prices[0];
    for (var j=1;j<n;j++) {
      dp[i][j] = Math.max(dp[i][j-1], profit+prices[j]);
      profit = Math.max(profit, dp[i-1][j-1]-prices[j]);
    }
  }
        
  return dp[k][n-1];
};
