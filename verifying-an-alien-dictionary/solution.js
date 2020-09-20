/**
 * @param {string[]} words
 * @param {string} order
 * @return {boolean}
 */
const alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
var isAlienSorted = function(words, order) {
  var map = new Map();
  for (var i=0;i<order.length;i++) {
    map.set(order[i], alphabet[i]);
  }
  
  var newwords = [...words].map((w, i) => ({ word: [...w].map((c) => map.get(c)).join(""), i }));

  var sorted = newwords.sort((a, b) => a.word.localeCompare(b.word));
  for (var i=0;i<sorted.length;i++) {
    if (sorted[i].i !== i) {
      return false;
    }
  }
  return true;
};
