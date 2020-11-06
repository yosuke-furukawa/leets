/**
 * @param {string[]} sentence
 * @param {number} rows
 * @param {number} cols
 * @return {number}
 */
var wordsTyping = function(sentence, rows, cols) {
  var s = sentence.join(" ") + " ";
  var start = 0, l = s.length;
  for (var i = 0; i < rows; i++) {
    start += cols;
    if (s.charAt(start % l) == ' ') {
      start++;
    } else {
      while (start > 0 && s.charAt((start-1) % l) != ' ') {
        start--;
      }
    }
  }
        
  return Math.floor(start / s.length);
};
