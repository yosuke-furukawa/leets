/**
 * @param {number[][]} grid
 * @return {number}
 */
var islandPerimeter = function(grid) {
  var zeros = 0;
  for (var h=0;h<grid.length;h++) {
    for (var w=0;w<grid[h].length;w++) {
      if (grid[h][w] === 1) {
        var north = grid[h-1] ? grid[h-1][w] : 0;
        var west = grid[h][w-1] || 0;
        var east = grid[h][w+1] || 0;
        var south = grid[h+1] ? grid[h+1][w] : 0;
        zeros += (4 - (north + west + east + south));
      }
    }
  }
  return zeros;
};
