/**
 * @param {number} n
 * @return {number[][]}
 */
var generateMatrix = function(n) {
  var matrix = new Array(n).fill(0).map(() => new Array(n).fill(0));
  
  var y = 0;
  var x = 0;
  var direction = "east";
  for (var i=1;i<=n**2;i++) {
    matrix[y][x] = i;
    if (direction === "east") {
      if (matrix[y][x+1] !== 0) {
        direction = "south";
        y++;
        continue;
      }
      x++;
    } else if (direction === "south") {
      if (matrix[y+1]?.[x] !== 0) {
        direction = "west";
        x--;
        continue;
      }
      y++;
    } else if (direction === "west") {
      if (matrix[y][x-1] !== 0) {
        direction = "north";
        y--;
        continue;
      }
      x--;
    } else if (direction === "north") {
      if (matrix[y-1]?.[x] !== 0) {
        direction = "east";
        x++;
        continue;
      }
      y--;
    }
  }
  return matrix;
};
