/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var topKFrequent = function(nums, k) {
  const map = {};
  const result = [];
  const bucket = Array(nums.length + 1).fill().map(() => []);

  for (let num of nums) {
    map[num] = ~~map[num] + 1;
  }

  for (let num in map) {
    bucket[map[num]].push(parseInt(num));
  }

  for (let i = nums.length; i >= 0 && k > 0; k--) {
    while (bucket[i].length === 0) i--;
    result.push(bucket[i].shift());
  }

  return result;
};
