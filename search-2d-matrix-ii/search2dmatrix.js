/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
var searchMatrix = function(matrix, target) {
  var x = 0;
  var y = matrix.length - 1;
  while(matrix[y] != null && matrix[y][x] != null) {
    if (matrix[y][x] < target) {
      x++;
    } else if (matrix[y][x] > target) {
      y--;
    } else {
      return true;
    }
  }
  return false;
};
