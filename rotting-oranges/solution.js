/**
 * @param {number[][]} grid
 * @return {number}
 */
var orangesRotting = function(grid) {
  var startpoint = [];
  var rots = 0;
  var oranges = 0;
  var points = [];
  startpoint.push(points);
  for (var y=0;y<grid.length;y++) {
    for (var x=0;x<grid[y].length;x++) {
      if (grid[y][x] === 2) {
        grid[y][x] = -grid[y][x];
        points.push([x, y]);
        rots += 1;
      }
      if (grid[y][x] === 1) {
        oranges += 1;
      }
    }
  }
  
  if (rots === 0 && oranges === 0) {
    return 0;
  }
  
  if (rots === 0 && oranges > 0) {
    return -1;
  }
  
  var prev = 0;
  var minutes = 0;
  while (prev !== rots) {
    var oranges = startpoint[minutes];
    if (oranges == null) {
      break;
    }
    var nextpoints = [];
    prev = rots;
    for (var orange of oranges) {
      var [x, y] = orange;
      var north = grid[y-1]?.[x];
      var east = grid[y][x+1];
      var west = grid[y][x-1];
      var south = grid[y+1]?.[x];
      if (north === 1) {
        grid[y-1][x] = -grid[y-1][x];
        nextpoints.push([x, y-1]);
        rots++;
      }
      if (east === 1) {
        grid[y][x+1] = -grid[y][x+1];
        nextpoints.push([x+1, y]);
        rots++;
      }
      if (west === 1) {
        grid[y][x-1] = -grid[y][x-1];
        nextpoints.push([x-1, y]);
        rots++;
      }
      if (south === 1) {
        grid[y+1][x] = -grid[y+1][x];
        nextpoints.push([x, y+1]);
        rots++;
      }
    }
    startpoint.push(nextpoints);
    minutes++;
  }
  
  for (var y=0;y<grid.length;y++) {
    for (var x=0;x<grid[y].length;x++) {
      if (grid[y][x] === 1) {
        return -1;
      }
    }
  }
  
  return minutes - 1;
};
