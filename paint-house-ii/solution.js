/**
 * @param {number[][]} costs
 * @return {number}
 */
var minCostII = function(costs) {
  if (costs.length === 0) {
    return 0;
  }
  var dp = new Array(costs.length+1).fill(0).map(() => new Array(costs[0].length).fill(0));
  
  for (var i=0;i<costs.length;i++) {
    for (var j=0;j<costs[i].length;j++) {
      var temp = [...dp[i]];
      temp.splice(j,1);
      dp[i+1][j] = temp.length > 0 ? (Math.min(...temp) + costs[i][j]) : costs[i][j];              
    }
  }
  return Math.min(...dp[dp.length-1]); 
};
