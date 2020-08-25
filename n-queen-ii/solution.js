
/**
 * @param {number} n
 * @return {number}
 */

var totalNQueens = function(n) {
  var board = new Array(n);
  for (var i=0;i<board.length;i++) {
    board[i] = new Array(n).fill(0);
  }
  var isSafe = function(row, col) {
    for (var r=0;r<row;r++) {
      if (board[r][col] === 1) {
        return false;
      }
    }
    for (var c=0;c<col;c++) {
      if (board[row][c] === 1) {
        return false;
      }
    }
    for (var i=0;i<n;i++) {
      if (board[row+i]?.[col+i] === 1) {
        return false;
      }
      if (board[row-i]?.[col-i] === 1) {
        return false;
      }
      if (board[row-i]?.[col+i] === 1) {
        return false;
      }
      if (board[row+i]?.[col-i] === 1) {
        return false;
      }
    }
    return true;
  }
  var backtrack = function(row, count) {
    for (var col = 0; col<n;col++) {
      if (isSafe(row, col)) {
        board[row][col] = 1;
        if (row + 1 === n) {
          count++;
        } else {
          count = backtrack(row+1, count);
        }
        board[row][col] = 0;
      }
    }
    return count;
  }
  return backtrack(0, 0);
};
