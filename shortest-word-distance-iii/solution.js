/**
 * @param {string[]} words
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var shortestWordDistance = function(words, word1, word2) {
  var word1List = words.map((w,i) => w===word1 ? i : -1).filter((n) => n>=0);
  var word2List = words.map((w,i) => w===word2 ? i : -1).filter((n) => n>=0);
  
  var min = Infinity;
  for (var i=0;i<word1List.length;i++) {
    var w1 = word1List[i];
    for (var j=0;j<word2List.length;j++) {
      var w2 = word2List[j];
      if (w1 === w2) {
        continue;
      }
      min = Math.min(min, Math.abs(w2 - w1));
      if (min === 1) {
        break;
      }
    }
  }
  return min;
};
