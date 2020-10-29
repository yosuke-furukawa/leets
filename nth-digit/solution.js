/**
 * @param {number} num
 * @return {number}
 */
var addDigits = function(num) {
  if (num === 0) {
    return 0;
  }
  return num % 9 || 9;
};
