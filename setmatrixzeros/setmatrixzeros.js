/**
 * @param {number[][]} matrix
 * @return {void} Do not return anything, modify matrix in-place instead.
 */
var setZeroes = function(matrix) {
  var modified = -Infinity
  for (var col = 0; col < matrix.length; col++) {
    for (var row = 0; row < matrix[col].length; row++) {
      if (matrix[col][row] === modified) {
        continue;
      }
      var n = matrix[col][row];
      if (n===0) {
        for (var r=0; r<matrix[col].length;r++) {
          if (matrix[col][r] !== 0) {
            matrix[col][r] = modified;
          }
        }
        for (var c=0; c<matrix.length;c++) {
          if (matrix[c][row] !== 0) {
            matrix[c][row] = modified;
          }
        }
      }
    }
  }
  for (var col = 0; col < matrix.length; col++) {
    for (var row = 0; row < matrix[col].length; row++) {
      if (matrix[col][row] === modified) {
        matrix[col][row] = 0;
      }
    }
  }
};
