/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var swap = (nums, i, j) => {
  [nums[j], nums[i]] = [nums[i], nums[j]];
}

var reverse = (nums, i, j) => {
  while(i < j) swap(nums, i++, j--);
}

var nextPermutation = function(nums) {
  if (nums.length < 2) {
    return;
  }
  for (var i=nums.length-2;i>=0;i--) {
    if (nums[i] < nums[i+1]) {
      break;
    }
  }

  if (i>=0) {
    var j = nums.length-1;
    while (j>0 && nums[j] <= nums[i]) {
      j--;
    }
    swap(nums, i, j);
  }
  reverse(nums, i + 1, nums.length - 1);
};
