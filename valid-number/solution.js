/**
 * @param {string} s
 * @return {boolean}
 */
var isNumber = function(s) {
  s = s.trim();
  if (s.length === 0) {
    return false;
  }
  return s == +s;
};
