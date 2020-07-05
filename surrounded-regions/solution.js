/**
 * @param {character[][]} board
 * @return {void} Do not return anything, modify board in-place instead.
 */
var willO = "o";
var hasNeighborO = (board, width, height) => {
  if (willO == board[height-1][width]) {
    board[height][width] = willO;
    return;
  } else if (willO == board[height+1][width]) {
    board[height][width] = willO;
    return;
  } else if (willO == board[height][width-1]) {
    board[height][width] = willO;
    return;
  } else if (willO == board[height][width+1]) {
    board[height][width] = willO;
    return;
  } else {
    board[height][width] = "X";
    return;
  }
}

var solve = function(board) {
  if (board.length === 0) {
    return;
  }
  // boarder
  var ws = 0;
  var we = board[0].length-1;
  var hs = 0;
  var he = board.length-1;
  var queue = [];
  var widthboard = [ws, we];
  for (var w of widthboard) {
    for (var h=hs;h<=he;h++) {
      var b = board[h][w];
      if (b === "O") {
        board[h][w] = willO;
        queue.push({w, h});
      }
    }
  }
  var heightboard = [hs, he];
  for (var h of heightboard) {
    for (var w=ws;w<=we;w++) {
      var b = board[h][w];
      if (b === "O") {
        board[h][w] = willO;
        queue.push({w, h});
      }
    }
  }
  while(queue.length > 0) {
    var {w, h} = queue.pop();
    var north = board[h-1] && board[h-1][w];
    var west = board[h][w-1];
    var east = board[h][w+1];
    var south = board[h+1] && board[h+1][w];
    if (north === "O") {
      board[h-1][w] = willO;
      queue.unshift({w, h:h-1});
    }
    if (west === "O") {
      board[h][w-1] = willO;
      queue.unshift({w:w-1, h});
    }
    if (east === "O") {
      board[h][w+1] = willO;
      queue.unshift({w:w+1, h});
    }
    if (south === "O") {
      board[h+1][w] = willO;
      queue.unshift({w, h:h+1});
    }
  }
  for (var h=0;h<board.length;h++) {
    for (var w=0;w<board[h].length;w++) {
      if (board[h][w] === willO) {
        board[h][w] = "O";
      } else {
        board[h][w] = "X";
      }
    }
  }
};
