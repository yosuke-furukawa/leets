/**
 * @param {string} J
 * @param {string} S
 * @return {number}
 */
var numJewelsInStones = function(J, S) {
  var count = 0;
  for (var s of [...S]) {
    if (J.includes(s)) {
      count++;
    }
  }
  return count;
};
