/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
var searchMatrix = function(matrix, target) {
  if (matrix.length === 0) {
    return false;
  }
  var row = 0;
  var col = 0;
  var maxrow = matrix.length;
  var maxcol = matrix[0].length;
  var set = new Set();
  while (row < maxrow && col < maxcol) {
    var key = `${row},${col}`;
    if (set.has(key)) {
      break;
    }
    // console.log({row, col, maxrow, maxcol})
    if (matrix[row][col] === target) {
      return true;
    }
    
    set.add(key);
    if (matrix[row][col] < target) {
      row = (row + 1);
      col = (col + 1) % maxcol;
      if (row >= maxrow) {
        row = maxrow - 1;
      }
    } else {
      row = (row - 1) % maxrow;
      if (row < 0) {
        break;
      }
    }
  }
  return false;
};
