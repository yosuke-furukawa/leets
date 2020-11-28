/**
 * @param {number[][]} costs
 * @return {number}
 */
var minCost = function(costs) {
  var dp = new Array(costs.length+1).fill(0).map(() => new Array(3).fill(0));
  
  for (var i=0;i<costs.length;i++) {
    for (var j=0;j<costs[i].length;j++) {
      if (j === 0) {
        dp[i+1][j] = Math.min(dp[i][1], dp[i][2]) + costs[i][j];              
      } else if (j === 1) {
        dp[i+1][j] = Math.min(dp[i][0], dp[i][2]) + costs[i][j];
      } else {
        dp[i+1][j] = Math.min(dp[i][0], dp[i][1]) + costs[i][j];
      }
    }
  }
  
  return Math.min(...dp[dp.length-1]);
};
