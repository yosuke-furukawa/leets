/**
 * @param {string[]} word1
 * @param {string[]} word2
 * @return {boolean}
 */
var arrayStringsAreEqual = function(word1, word2) {
  var str2 = word2.reduce((cur, acc) => cur+acc, "");
  var index = 0;
  for (var s of word1) {
    for (var c of s) {
      if (index >= str2.length || c !== str2.charAt(index)) {
        return false;
      }
      index++;
    }
  }
  return index === str2.length;
};
