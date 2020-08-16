/**
 * // This is the master's API interface.
 * // You should not implement it, or speculate about its implementation
 * function Master() {
 *
 *     @param {string[]} wordlist
 *     @param {Master} master
 *     @return {integer}
 *     this.guess = function(word) {
 *         ...
 *     };
 * };
 */
/**
 * @param {string[]} wordlist
 * @param {Master} master
 * @return {void}
 */
var findSecretWord = function(wordlist, master) {
  var words = [...wordlist];
  var word = words[Math.floor(Math.random() * words.length)];
  var wordSet = new Set();
  wordSet.add(word);
  var n = master.guess(word);
  if (n === word.length) {
    return;
  }
  
  function pickupNext(word, n) {
    var newwords = [];
    for (var w of words) {
      if (wordSet.has(w)) {
        continue;
      }
      var similarity = 0;
      for (var i=0;i<w.length;i++) {
        if (w[i] === word[i]) {
          similarity++;
        }
      }
      if (similarity === n) {
        newwords.push(w);
      }
    }
    words = newwords;
  }
  
  for(var i=0;i<9;i++) {
    pickupNext(word, n);
    word = words[Math.floor(Math.random() * words.length)];
    wordSet.add(word);
    n = master.guess(word);
    if (n === word.length) {
      return;
    }
  }
};
