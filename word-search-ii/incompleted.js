/**
 * @param {character[][]} board
 * @param {string[]} words
 * @return {string[]}
 */

var findWords = function(board, words) {
  var result = [];
  var cache = {};
  var backtrack = function(board, word, i, h, w, visited) {
    visited.set(`${h}, ${w}`, true);
    if (word[i] == null) {
      return true;
    }
    var up = board[h-1] != null && board[h-1][w];
    var down = board[h+1] != null && board[h+1][w];
    var left = board[h][w-1];
    var right = board[h][w+1];
    var u,d,l,r;
    if (up === word[i] && !visited.has(`${h-1}, ${w}`)) {
      u = backtrack(board, word, i+1, h-1, w, new Map(visited));
    }
    if (down === word[i] && !visited.has(`${h+1}, ${w}`)) {
      d = backtrack(board, word, i+1, h+1, w, new Map(visited));
    }
    if (left === word[i] && !visited.has(`${h}, ${w-1}`)) {
      l = backtrack(board, word, i+1, h, w-1, new Map(visited));
    }
    if (right === word[i] && !visited.has(`${h}, ${w+1}`)) {
      r = backtrack(board, word, i+1, h, w+1, new Map(visited));
    }
    cache[word.substring(0, i)] = { h, w };
    return u || d || l || r;
  }
  for (const word of words) {
    var i=0;
    var exists = false;
    for (var h=0;h<board.length;h++) {
      for (var w=0;w<board[h].length;w++) {
        if (word[i] === board[h][w]) {
          exists = backtrack(board, word, i+1, h, w, new Map());
          console.log(cache);
          if (exists && !result.includes(word)) {
            result.push(word);
          }
        }
      }
    }
  }
  return result;
};
