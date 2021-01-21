/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var mostCompetitive = function(nums, k) {
  var res = [];
  var popLimit = nums.length-k;
    
  for(var it=0;it<nums.length;it++) {
    while(res.length && res[res.length-1] > nums[it] && popLimit) {
      res.pop();
      popLimit--;
    }
    res.push(nums[it]);
  }
    
  return res.slice(0, k);
};
