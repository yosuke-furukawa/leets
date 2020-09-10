var swap = function(nums, i, j) {
  [nums[j], nums[i]] = [nums[i], nums[j]];
  return nums;
}

var backtrack = function(nums, l, r, ans, set) {
  if (l === r) {
    if (!set.has(nums.join(","))) {
      set.add(nums.join(","));
      ans.push([...nums]);
    }
  } else {
    for (var i=l; i<=r; i++) {
      nums = swap(nums, l, i);
      backtrack(nums, l+1, r, ans, set); 
      nums = swap(nums, l, i);
    }
  }
};

/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permuteUnique = function(nums) {
  var ans = [];
  backtrack(nums, 0, nums.length-1, ans, new Set());
  return ans; 
};
