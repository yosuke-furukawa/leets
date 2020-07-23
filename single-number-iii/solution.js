/**
 * @param {number[]} nums
 * @return {number[]}
 */
var singleNumber = function(nums) {
  var set = new Set();
  for (var i=0;i<nums.length;i++) {
    var n = nums[i];
    if (set.has(n)) {
      set.delete(n);
    } else {
      set.add(nums[i]);
    }
  }
  return Array.from(set);
};
