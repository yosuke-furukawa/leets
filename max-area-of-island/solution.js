/**
 * @param {number[][]} grid
 * @return {number}
 */
var maxAreaOfIsland = function(grid) {
  var set = new Set();
  var max = 0;
  for (var i=0;i<grid.length;i++) {
    for (var j=0;j<grid[i].length;j++) {
      var key = `${i},${j}`;
      if (set.has(key)) {
        continue;
      }
      if (grid[i][j] === 0) {
        continue;
      }
      if (grid[i][j] === 1) {
        var queue = [[i, j]];
        var count = 0;
        while(queue.length > 0) {
          var [x, y] = queue.shift();
          var key = `${x},${y}`;
          if (set.has(key)) {
            continue;
          }
          set.add(key);
          count++;
          var north = grid[x-1]?.[y];
          if (north === 1) {
            queue.push([x-1, y]);
          }
          var east = grid[x][y+1];
          if (east === 1) {
            queue.push([x, y+1]);
          }

          var south = grid[x+1]?.[y];
          if (south === 1) {
            queue.push([x+1, y]);
          }
          var west = grid[x][y-1];
          if (west === 1) {
            queue.push([x, y-1]);
          }
        }
        
        max = Math.max(max, count);
      }
    }
  }
  return max;
};
