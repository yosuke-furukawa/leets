/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var subarraySum = function(nums, k) {
  var map1 = new Map();
  var map2 = new Map();
  var count = 0;

  var sum = 0;
  for (var i=0;i<nums.length;i++) {
    sum += nums[i];
    map1.set(sum, i);
    map2.set(i, sum);
    if (sum === k) {
      count++;
    }
  }
  
  for (var i=0;i<nums.length;i++) {
    for (var j=i+1;j<nums.length;j++) {
      var target = map2.get(j) - map2.get(i);
      if (target === k) {
        count++;
      }
    }
  }
  return count;
};
