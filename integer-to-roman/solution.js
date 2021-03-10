/**
 * @param {number} num
 * @return {string}
 */

var intToRoman = function(num) {
  var thousands = ["", "M", "MM", "MMM"];
  var hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"]; 
  var tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
  var ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
  return thousands[(num / 1000) | 0] + hundreds[(num % 1000 / 100) | 0] + tens[(num % 100 / 10) | 0] + ones[num % 10];
};
