/**
 * @param {number[][]} grid
 * @return {number}
 */
var cherryPickup = function(grid) {
  var m = grid.length;
  var n = grid[0].length;
  var dpCache = new Array(m).fill(0).map(() => new Array(n).fill(0).map(() => new Array(n).fill(0)));
  
  for (var i=0;i<m;i++) {
    for (var j=0;j<n;j++) {
      for (var k=0;k<n;k++) {
        dpCache[i][j][k] = -1;
      }
    }
  }
  
  return dp(0, 0, n-1, grid, dpCache);
};

function dp(row, col1, col2, grid, dpCache) {
  if (col1 < 0 || col1 >= grid[0].length || col2 < 0 || col2 >= grid[0].length) {
    return 0;
  }
 
  if (dpCache[row][col1][col2] !== -1) {
    return dpCache[row][col1][col2];
  }
  
  var result = 0;
  result += grid[row][col1];
  if (col1 !== col2) {
    result += grid[row][col2];
  }
  
  if (row !== grid.length - 1) {
    var max = 0;
    for (var newCol1 = col1 - 1;newCol1 <= col1 + 1; newCol1++) {
      for (var newCol2 = col2 - 1; newCol2 <= col2 + 1; newCol2++) {
        max = Math.max(max, dp(row + 1, newCol1, newCol2, grid, dpCache));
      }
    }
    result += max;
  }
  
  dpCache[row][col1][col2] = result;
  return result;
}
