/**
 * @param {number} poured
 * @param {number} query_row
 * @param {number} query_glass
 * @return {number}
 */
var champagneTower = function(poured, query_row, query_glass) {
  var dp = new Array(101).fill(0);
  dp[0] = poured;
  for (var i=1;i<=query_row;i++) {
    for (var j=i;j>=0;j--) {
      dp[j] = Math.max(0.0, (dp[j] - 1) / 2.0);
      dp[j+1] += dp[j];
      // console.log(dp[j], dp[j+1], j);
    }
  }
  return Math.min(1.0, dp[query_glass]);
};
