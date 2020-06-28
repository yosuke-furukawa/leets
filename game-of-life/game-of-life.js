/**
 * @param {number[][]} board
 * @return {void} Do not return anything, modify board in-place instead.
 */
var gameOfLife = function(board) {
  var will0 = -1;
  var will1 = 2;
  for (var col=0;col<board.length; col++) {
    for (var row=0;row<board[col].length;row++) {
      var target = board[col][row];
      var neighbors = [];
      for (var c of [col-1,col,col+1]) {
        for (var r of [row-1,row,row+1]) {
          if (c === col && r === row) {
            continue;
          }
          if (board[c] && board[c][r] != null) {
            neighbors.push(board[c][r]);
          }
        }
      }
      if (target === 0) {
        var alivecells = 0;
        for (const neighbor of neighbors) {
          if (neighbor === 1 || neighbor === will0) {
            alivecells++;
          }
        }
        if (alivecells === 3) {
          board[col][row] = will1;
        }
      } else {
        var alivecells = 0;
        for (const neighbor of neighbors) {
          if (neighbor === 1 || neighbor === will0) {
            alivecells++;
          }
        }
        if (alivecells > 3) {
          board[col][row] = will0;
        } else if (alivecells >= 2) {
          board[col][row] = 1;
        } else {
          board[col][row] = will0;
        }
      }
    }
  }
  for (var col=0;col<board.length; col++) {
    for (var row=0;row<board[col].length;row++) {
      if (board[col][row] === will0) {
        board[col][row] = 0;
      } else if (board[col][row] === will1) {
        board[col][row] = 1;
      }
    }
  }
};
