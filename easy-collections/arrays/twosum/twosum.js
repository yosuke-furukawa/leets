var twoSum = function(nums, target) {
  var map = new Map();
  for (var i=0; i<nums.length; i++) {
    var x = nums[i];
    var value = target - x;
    if (map.has(value)) {
      return [map.get(value), i];
    }
    map.set(nums[i], i);
  }
};

console.log(twoSum([2, 7, 11, 15], 9));
