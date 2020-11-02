/**
 * @param {string} s
 * @return {string}
 */
var frequencySort = function(s) {
  var frequency = new Map();
  for (var c of [...s]) {
    if (frequency.has(c)) {
      frequency.set(c, frequency.get(c) + 1);
    } else {
      frequency.set(c, 1);
    }
  }
  
  return [...s].sort((a, b) => { 
    var diff = frequency.get(b) - frequency.get(a);
    if (diff === 0) {
      return a.localeCompare(b);
    }
    return diff;
  }).join("");
};
