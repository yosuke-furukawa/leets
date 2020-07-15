class TrieNode {
  constructor() {
    this.children = new Map();
    this.word = null;
  }
}

var findWords = function(board, words) {
  var result = [];
  var _board = new Array(board.length);
  for (var i=0;i<_board.length;i++){
    _board[i] = [...board[i]];
  }
  // Step 1). Construct the Trie
  const root = new TrieNode();
  for (const word of words) {
    var node = root;
    for (const char of word) {
      if (node.children.has(char)) {
        node = node.children.get(char);
      } else {
        const newNode = new TrieNode();
        node.children.set(char, newNode);
        node = newNode;
      }
    }
    node.word = word;  // store words in Trie
  }
  
  function backtrack(col, row, parent) {
    const char = _board[col][row];
    const currNode = parent.children.get(char);
    if (currNode == null) {
      return;
    }
    // check if there is any match
    if (currNode?.word != null) {
      result.push(currNode.word);
      currNode.word = null;
    }

    // mark the current letter before the EXPLORATION
    _board[col][row] = '#';

    // explore neighbor cells in around-clock directions: up, right, down, left
    var rowOffset = [-1, 0, 1, 0];
    var colOffset = [0, 1, 0, -1];
    for (var i = 0; i < 4; ++i) {
      var newRow = row + rowOffset[i];
      var newCol = col + colOffset[i];
      if (newCol < 0 || newCol >= _board.length || newRow < 0
          || newRow >= _board[0].length) {
        continue;
      }
      if (currNode.children.has(_board[newCol][newRow])) {
        backtrack(newCol, newRow, currNode);
      }
    }

    // End of EXPLORATION, restore the original letter in the board.
    _board[col][row] = char;

    // Optimization: incrementally remove the leaf nodes
    if (currNode.children.size == 0) {
      parent.children.delete(char);
    }
  }
  
  
  for (var col=0;col<_board.length;col++) {
    for (var row=0;row<_board[col].length;row++) {
      if (root.children.has(_board[col][row])) {
        backtrack(col, row, root);
      }
    }
  }

  return result;
};
