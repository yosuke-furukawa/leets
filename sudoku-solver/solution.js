/**
 * @param {character[][]} board
 * @return {void} Do not return anything, modify board in-place instead.
 */
var solveSudoku = function(board) {
  const N = board.length;
  var findUnassigned = () => {
    for (var nx=0;nx<N;nx++) {
      for (var ny=0;ny<N;ny++) {
        if (board[nx][ny] === ".") {
          return [nx, ny];
        }
      }
    }
    return false;
  }
  
  var isValid = (x, y, num) => {
    for (var xi=0;xi<N;xi++) {
      if (board[xi][y] === "" + num) {
        return false;
      }
    }
    
    for (var yi=0;yi<N;yi++) {
      if (board[x][yi] === "" + num) {
        return false;
      }
    }
    var boxx = Math.floor(x/3);
    var boxy = Math.floor(y/3);
    for (var xi=boxx*3;xi<boxx*3+3;xi++) {
      for (var yi=boxy*3;yi<boxy*3+3;yi++) {
        if (board[xi][yi] === "" + num) {
          return false;
        }
      } 
    }
    return true;
  }
  var solve = () => {
    var unassigned = findUnassigned();
    if (!unassigned) {
      return true;
    }
    let [x, y] = unassigned;
    for (var num=1;num<=9;num++) {
      if (isValid(x, y, num)) {
        board[x][y] = "" + num;
        var solved = solve();
        if (solved) {
          return true;
        }
        board[x][y] = ".";
      }
    }
    return false;
  };
  solve();
};
