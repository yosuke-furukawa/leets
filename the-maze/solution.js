/**
 * @param {number[][]} maze
 * @param {number[]} start
 * @param {number[]} destination
 * @return {boolean}
 */


var hasPath = function(maze, start, destination) {
  var visited = new Set();
  var [destx, desty] = destination;
  
  var tilt = (x, y, directions) => {
    var x1 = x;
    var y1 = y;
    var [dirx, diry] = directions;
    
    if (x1 + dirx >= maze.length) {
      return [x1, y1];
    }
    
    if (x1 + dirx < 0) {
      return [x1, y1];
    }
    
    if (y1 + diry >= maze[0].length) {
      return [x1, y1];
    }
    
    if (y1 + diry < 0) {
      return [x1, y1];
    }
    
    if (maze[x1 + dirx] && maze[x1 + dirx][y1 + diry] !== 0) {
      return [x1, y1];
    }
    
    while (maze[x1 + dirx] && maze[x1 + dirx][y1 + diry] === 0) {
      x1 += dirx;
      y1 += diry;
    }
    
    return [x1, y1];
  }
  
  var helper = (x, y) => {
    if (x === destx && y === desty) {
      return true;
    }
    var key = `${x},${y}`;
    if (visited.has(key)) {
      return false;
    } else {
      visited.add(key);
    }
    
    var northTo = tilt(x, y, [0, -1]);
    var southTo = tilt(x, y, [0, 1]);
    var eastTo = tilt(x, y, [1, 0]);
    var westTo = tilt(x, y, [-1, 0]);
    return helper(...northTo) || helper(...southTo) || helper(...eastTo) || helper(...westTo);
  };
  
  return helper(...start);
};
