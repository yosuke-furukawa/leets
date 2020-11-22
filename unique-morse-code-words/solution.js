/**
 * @param {string[]} words
 * @return {number}
 */

const morses = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];

var uniqueMorseRepresentations = function(words) {
  var morseWords = new Set();
  for (var word of words) {
    var morseWord = [...word].map((w) => morses[w.charCodeAt() - 97]).join("");
    morseWords.add(morseWord);
  }
  
  return morseWords.size;
};
