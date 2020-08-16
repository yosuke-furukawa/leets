/**
 * @param {number[]} nums
 * @return {number}
 */
var findNumbers = function(nums) {
  var count = 0;
  for (var n of nums) {
    var digits = 0;
    while (n >= 1) {
      n = n / 10;
      digits++;
    }
    if (digits % 2 === 0) {
      count++;
    }
  }
  return count;
};
