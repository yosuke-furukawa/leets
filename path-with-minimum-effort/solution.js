/**
 * @param {number[][]} heights
 * @return {number}
 */
class Cell {
  constructor(x, y, diff) {
    this.x = x;
    this.y = y;
    this.diff = diff;
  }
}

var minimumEffortPath = function(heights) {
  var dx = [-1,0,1,0];
  var dy = [0,1,0,-1];
  var row = heights.length;
  var col = heights[0].length;
  
  var differenceMatrix = Array(row).fill(0).map(() => Array(col).fill(0));
  
  for (let eachRow of differenceMatrix) {
    eachRow.fill(Infinity);
  }
  
  differenceMatrix[0][0] = 0;
  var queue = new MinPriorityQueue();
  var visited = Array(row).fill(0).map(() => Array(col).fill(false));
  queue.enqueue(new Cell(0,0,0), 1);
  
  while (!queue.isEmpty()) {
    var { element: curr } = queue.dequeue();
    visited[curr.x][curr.y] = true;
    if (curr.x === row-1 && curr.y === col-1) {
      return curr.diff;
    }
    
    for (var i=0;i<4;i++) {
      var ax = curr.x + dx[i];
      var ay = curr.y + dy[i];
      if (ax >= 0 && ax <= row-1 && ay >= 0 && ay <= col-1) {
        var currentDiff = Math.abs(heights[ax][ay] - heights[curr.x][curr.y]);
        var maxDiff = Math.max(currentDiff, differenceMatrix[curr.x][curr.y]);
        
        if (differenceMatrix[ax][ay] > maxDiff) {
          differenceMatrix[ax][ay] = maxDiff;
          queue.enqueue(new Cell(ax, ay, maxDiff), maxDiff+1);
        }
      }
    }
  }
  return differenceMatrix[row-1][col-1];
};
