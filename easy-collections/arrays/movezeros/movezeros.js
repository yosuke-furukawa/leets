var moveZeroes = function(nums) {
  var zeros = 0;
  for (var i = 0; i < nums.length;) {
    if (nums[i] === 0) {
      nums.splice(i, 1);
      zeros++;
      continue;
    }
    i++;
  }
  for (var i = 0; i < zeros; i++) {
    nums.push(0);
  }
};

