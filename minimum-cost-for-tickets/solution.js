/**
 * @param {number[]} days
 * @param {number[]} costs
 * @return {number}
 */
var mincostTickets = function(days, costs) {
  var dp = new Array(366).fill(0);
  var j = 0;
  for (var i=1;i<366;i++) {
    dp[i] = costs[0] + dp[i-1];
    
    if (i >= 7) {
      dp[i] = Math.min(costs[1] + dp[i-7], dp[i]);
    }
    
    if (i >= 30) {
      dp[i] = Math.min(costs[2] + dp[i-30], dp[i]);
    }
    
    if (j < days.length && days[j] === i) {
      j++;
    } else {
      dp[i] = Math.min(dp[i], dp[i-1]);
    }
  }
  
  return dp[365];
};
