/**
 * @param {number} n
 * @return {string}
 */

var table = {};
for (var i=0;i<26;i++) {
  table[i+1] = String.fromCharCode(i+65);
}
table[0] = table[26];
var convertToTitle = function(n) {
  var result = "";
  while (n > 26) {
    var divisor = parseInt(n / 26);
    var reminder = n % 26;
    result = table[reminder] + result;
    if (reminder === 0) {
      divisor--;
    }
    n = divisor;
  }
  if (n > 0) {
    result = table[n] + result;
  }
  return result;
};
