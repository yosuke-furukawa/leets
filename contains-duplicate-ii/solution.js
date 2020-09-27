/**
 * @param {number[]} nums
 * @param {number} k
 * @return {boolean}
 */
var containsNearbyDuplicate = function(nums, k) {
  var map = new Map();
  for (var i=0;i<nums.length;i++) {
    var num = nums[i];
    if (map.has(num)) {
      if (i - map.get(num) <= k) {
        return true;
      } else {
        map.set(num, i);
      }
    } else {
      map.set(num, i);
    }
  }
  return false;
};
