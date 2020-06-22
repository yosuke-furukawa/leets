/**
 * @param {character[][]} board
 * @param {string} word
 * @return {boolean}
 */
var exist = function(board, word) {
  var rowlen = board.length;
  var collen = board[0].length;
  var backtrack = function(row, col, word, index) {
    if (index >= word.length) {
      return true;
    }
    if (row < 0 
        || row === rowlen 
        || col < 0 
        || col === collen
        || board[row][col] != word[index]) {
      return false;
    }
    var result = false;
    board[row][col] = "#";
    var rowOffsets = [0, 1, 0, -1];
    var colOffsets = [1, 0, -1, 0];
    for (var d = 0; d < 4; d++) {
      result = backtrack(row + rowOffsets[d], col + colOffsets[d], word, index + 1);
      if (result) {
        break;
      }
    }
    board[row][col] = word[index];
    return result;
  }
  for (var row = 0; row < rowlen; row++) {
    for (var col = 0; col < collen; col++) {
      if (backtrack(row, col, word, 0)) {
          return true;
      }
    }
  }
  return false;
};

