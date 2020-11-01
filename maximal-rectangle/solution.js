/**
 * @param {character[][]} matrix
 * @return {number}
 */
var maximalRectangle = function(matrix) {
  if (matrix.length <= 0) return 0;
  const n = matrix.length;
  const m = matrix[0].length;
  const dp = new Array(n).fill([]).map(() => new Array(m).fill(0))
  let maxArea = 0;
  for (let r = 0; r < n; r++) {
    for (let c = 0; c < m; c++) {
      if (matrix[r][c] == 0) continue;
      dp[r][c] += dp[r - 1] ? dp[r - 1][c] + 1 : 1;
      let min = dp[r][c];
      for (let k = c; k >= 0; k--) {
        if (dp[r][k] == 0) break;
        min = dp[r][k] < min ? dp[r][k] : min;
        maxArea = Math.max(maxArea, min * (c - k + 1))
      }
    }
  }
  return maxArea;
};
