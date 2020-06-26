/**
 * Initialize your data structure here.
 * @param {number} n
 */
var TicTacToe = function(n) {
  this.n = n;
  this.rows = new Array(n).fill("");
  this.cols = new Array(n).fill("");
  this.win1 = "1".repeat(n);
  this.win2 = "2".repeat(n);
  this.diagonal = "";
  this.antiDiagonal = "";
};

/**
 * Player {player} makes a move at ({row}, {col}).
        @param row The row of the board.
        @param col The column of the board.
        @param player The player, can be either 1 or 2.
        @return The current winning condition, can be either:
                0: No one wins.
                1: Player 1 wins.
                2: Player 2 wins. 
 * @param {number} row 
 * @param {number} col 
 * @param {number} player
 * @return {number}
 */
TicTacToe.prototype.move = function(row, col, player) {
  this.rows[row] += String(player);
  this.cols[col] += String(player);
  this.diagonal += row === col ? String(player) : "";
  this.antiDiagonal += (row + col) === this.n-1 ? String(player) : "";
  var winnings = player === 1 ? this.win1 : this.win2; 
  if (this.rows[row] === winnings || this.cols[col] === winnings) {
    return player;
  }
  if (this.diagonal === winnings || this.antiDiagonal === winnings) {
    return player;
  }
  return 0;
};

/** 
 * Your TicTacToe object will be instantiated and called as such:
 * var obj = new TicTacToe(n)
 * var param_1 = obj.move(row,col,player)
 */
