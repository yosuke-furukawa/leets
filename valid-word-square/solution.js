/**
 * @param {string[]} words
 * @return {boolean}
 */
var validWordSquare = function(words) {
  var rows = words.length;
  var cols = words.reduce((acc, w) => Math.max(acc,w.length), 0);
  for (var row=0;row<rows;row++) {
    for (var col=0;col<cols;col++) {
      if (words[row]?.[col] !== words[col]?.[row]) {
        return false;
      }
    }
  }
  
  return true;
};
