/**
 * @param {number[]} nums
 * @return {number[]}
 */
var findDisappearedNumbers = function(nums) {
  var range = [...Array(nums.length+1).keys()].slice(1);
  var set = new Set(range);
  for (var n of nums) {
    set.delete(n);
  }
  return [...set];
};
