/**
 * @param {string[]} words
 */
var WordDistance = function(words) {
  this.words = words;
};

/** 
 * @param {string} word1 
 * @param {string} word2
 * @return {number}
 */
WordDistance.prototype.shortest = function(word1, word2) {
  var min = Infinity;
  var word1pos;
  var word2pos;
  for (var i=0;i<this.words.length;i++) {
    var w = this.words[i];
    if (w !== word1 && w !== word2) {
      continue;
    }
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

/** 
 * Your WordDistance object will be instantiated and called as such:
 * var obj = new WordDistance(words)
 * var param_1 = obj.shortest(word1,word2)
 */
