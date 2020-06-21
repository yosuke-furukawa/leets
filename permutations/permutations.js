var swap = function(nums, i, j) {
  [nums[j], nums[i]] = [nums[i], nums[j]];
  return nums;
}

var backtrack = function(nums, l, r, ans) {
  if (l === r) {
    ans.push([...nums]);
  } else {
    for (var i=l; i<=r; i++) {
      nums = swap(nums, l, i, ans);
      backtrack(nums, l+1, r, ans); 
      nums = swap(nums, l, i, ans);
    }
  }
};


/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permute = function(nums) {
  var ans = [];
  backtrack(nums, 0, nums.length-1, ans);
  return ans;
};

