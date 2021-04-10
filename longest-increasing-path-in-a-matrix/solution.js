/**
 * @param {number[][]} matrix
 * @return {number}
 */
var dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];


function dfs(matrix, i, j, m, n, cache) {
  if (cache[i][j] != null) return cache[i][j];
  var max = 1;
  for(var dir of dirs) {
    var x = i + dir[0];
    var y = j + dir[1];
    if(x < 0 || x >= m || y < 0 || y >= n || matrix[x][y] <= matrix[i][j]) continue;
    var len = 1 + dfs(matrix, x, y, m, n, cache);
    max = Math.max(max, len);
  }
  cache[i][j] = max;
  return max;
}
var longestIncreasingPath = function(matrix) {
  if (matrix.length == 0) return 0;
  var m = matrix.length;
  var n = matrix[0].length;
  var cache = new Array(m);
  for (var i=0;i<m;i++) {
    cache[i] = new Array(n);
  }
  var max = 1;
  for(var i = 0; i < m; i++) {
    for(var j = 0; j < n; j++) {
      var len = dfs(matrix, i, j, m, n, cache);
      
      max = Math.max(max, len);
    }
  }   
  return max;
};
