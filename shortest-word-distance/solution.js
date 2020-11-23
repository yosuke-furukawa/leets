/**
 * @param {string[]} words
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var shortestDistance = function(words, word1, word2) {
  var min = Infinity;
  var word1pos;
  var word2pos;
  for (var i=0;i<words.length;i++) {
    var w = words[i];
    if (w !== word1 && w !== word2) {
      continue;
    }
    // console.log({w, word1, word2, word1pos, word2pos})
    if (w === word1) {
      if (word2pos >= 0) {
        min = Math.min(min, Math.abs(i - word2pos));
      }
      word1pos = i;
    }
    if (w === word2) {
      if (word1pos >= 0) {
        min = Math.min(min, Math.abs(i - word1pos));
      }
      word2pos = i;
    }
    if (min === 1) {
      break;
    }
  }
  return min;
};
