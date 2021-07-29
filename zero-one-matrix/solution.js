/**
 * @param {number[][]} matrix
 * @return {number[][]}
 */
var updateMatrix = function(matrix) {
  const queue = [];
  for (var x=0;x<matrix.length;x++) {
    for (var y=0;y<matrix[x].length;y++) {
      if (matrix[x][y] === 1) {
        queue.push([x, y]);
      }
    }
  }
  while (queue.length > 0) {
    const [x, y] = queue.shift();
    var prev = matrix[x][y];
    var north = matrix[x-1]?.[y];
    var south = matrix[x+1]?.[y];
    var west = matrix[x][y-1];
    var east = matrix[x][y+1];
    var cost = Math.min(...[north, south, west, east].filter((x) => x != null));
    if (cost === 0 || cost + 1 === prev) {
      continue;
    }
    
    matrix[x][y] = cost + 1;
    queue.push([x, y]);
  }
  return matrix;
};
