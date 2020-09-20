/**
 * @param {number[][]} grid
 * @return {number}
 */
var uniquePathsIII = function(grid) {
  var pathnum = 0;
  var startpoint = [];
  
  for (var i=0;i<grid.length;i++) {
    for (var j=0;j<grid[i].length;j++) {
      if (grid[i][j] !== -1) {
        pathnum++;
      }
      if (grid[i][j] === 1) {
        startpoint = [i, j];
      }
    }
  }
  
  if (startpoint.length === 0) {
    return 0;
  }
  
  var result = 0;
  var key = (pos) => {
    return `(${pos[0]},${pos[1]})`;
  }
  var isValid = (pos, path) => {
    if (pos[0] < 0 || pos[1] < 0) {
      return false;
    }
    
    if (pos[0] >= grid.length || pos[1] >= grid[0].length) {
      return false;
    }
    
    if (grid[pos[0]][pos[1]] === -1) {
      return false;
    }

    if (path.has(key(pos))) {
      return false;
    }
    
    return true;
  }
  
 
  
  var backtrack = (currentPos, path) => {
    // console.log(currentPos, path);
    if (grid[currentPos[0]][currentPos[1]] === 2 && path.size === pathnum) {
      result++;
      return;
    }
    
    var north = [currentPos[0] - 1, currentPos[1]];
    var east = [currentPos[0], currentPos[1] + 1];
    var west = [currentPos[0], currentPos[1] - 1];
    var south = [currentPos[0] + 1, currentPos[1]];
    if (isValid(north, path)) {
      var northkey = key(north);
      path.add(northkey);
      backtrack(north, path);
      path.delete(northkey);
    }
    
    if (isValid(east, path)) {
      var eastkey = key(east);
      path.add(eastkey);
      backtrack(east, path);
      path.delete(eastkey);
    }
    
    if (isValid(west, path)) {
      var westkey = key(west);
      path.add(westkey);
      backtrack(west, path);
      path.delete(westkey);
    }
    
    if (isValid(south, path)) {
      var southkey = key(south);
      path.add(southkey);
      backtrack(south, path);
      path.delete(southkey);
    }
  };
  
  
  var path = new Set();
  path.add(key(startpoint));
  backtrack(startpoint, path);
  return result;
};
