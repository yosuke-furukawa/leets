/** 
 * Forward declaration of guess API.
 * @param {number} num   your guess
 * @return 	            -1 if num is lower than the guess number
 *			             1 if num is higher than the guess number
 *                       otherwise return 0
 * var guess = function(num) {}
 */

/**
 * @param {number} n
 * @return {number}
 */
var guessNumber = function(n) {
  var l = 1;
  var r = n;
  var ans = Math.floor((l+r)/2);
  while (l <= r) {
    var g = guess(ans);

    if (g === 0) {
      return ans;
    }
    if (g > 0) {
      l = ans + 1;
    } else {
      r = ans - 1;
    }
    ans = Math.floor((l+r)/2);
  }
  return ans <= 0 ? 1 : ans;
};
