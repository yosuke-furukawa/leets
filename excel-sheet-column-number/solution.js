/**
 * @param {string} s
 * @return {number}
 */
var titleToNumber = function(s) {
  var nums = [...s].map((c) => c.charCodeAt()-64);
  var result = 0;
  var count = 0;
  while(nums.length > 0) {
    var n = nums.pop();
    result += (26 ** count) * n;
    count++;
  }
  return result;
};
