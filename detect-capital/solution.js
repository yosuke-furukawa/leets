/**
 * @param {string} word
 * @return {boolean}
 */
var detectCapitalUse = function(word) {
  return Boolean(word.match(/^[A-Z]+$/g) || word.match(/^[a-z]+$/g) || word.match(/^[A-Z][a-z]+$/));
};
