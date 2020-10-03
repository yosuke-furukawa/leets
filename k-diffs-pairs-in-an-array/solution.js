/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var findPairs = function(nums, k) {
  var map = new Map();
  for (var i=0;i<nums.length;i++) {
    var n = nums[i];
    if (map.has(n)) {
      map.get(n).push(i);
      continue;
    }
    map.set(n, [i]);
  }
  
  var results = 0;
  for (var key of map.keys()) {
    var arrs = map.get(key);
    var targetPlus = k + key;
    
    if (map.has(targetPlus) && map.get(targetPlus) !== arrs) {
      results += 1;
    } else if (map.has(targetPlus) && map.get(targetPlus).length > 1) {
      results += 1;
    }
  }
  return results;
};
