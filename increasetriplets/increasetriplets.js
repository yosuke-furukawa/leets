/**
 * @param {number[]} nums
 * @return {boolean}
 */
var increasingTriplet = function(nums) {
  var lo = Infinity;
  var mi = Infinity;
  for(var n = 0;n < nums.length;n++) {
    var a = nums[n];
    if (a < lo) {
      lo = a;
    }
    if (lo < a && a < mi) {
      mi = a;
    }
    if (mi < a) {
      return true;
    }
  }
  return false;
};
