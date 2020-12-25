/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
  var str = String(x);
  for (var i=0;i<Math.floor(str.length/2);i++) {
    if (str[i] !== str[str.length-1-i]) {
      return false;
    }
  }
  return true;
};
