/**
 * @param {string[]} word1
 * @param {string[]} word2
 * @return {boolean}
 */
var arrayStringsAreEqual = function(word1, word2) {
  var sentences1 = word1.reduce((cur, acc) => cur+acc, "");
  var sentences2 = word2.reduce((cur, acc) => cur+acc, "");
  return sentences1 === sentences2;
};
