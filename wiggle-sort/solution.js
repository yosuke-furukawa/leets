/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
const wiggleSort = nums => {
  for (let i = 1; i < nums.length; i++) {
    if ((i % 2 === 0) === nums[i] > nums[i - 1]) {
      [nums[i], nums[i - 1]] = [nums[i - 1], nums[i]]; // swap i, i-1
    }
  }
};
