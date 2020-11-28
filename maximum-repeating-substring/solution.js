/**
 * @param {string} sequence
 * @param {string} word
 * @return {number}
 */
var maxRepeating = function(sequence, word) {
  var count = 0;
  var result = 0;
  for (var i=0;i<sequence.length;i++) {
    var w = sequence.slice(i, i+word.length);
    // console.log(w, i);
    if (word === w) {
      count++;
      i += word.length-1;
      result = Math.max(result, count);
    } else {
      count = 0;
    }
  }
  return result;
};
