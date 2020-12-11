/**
 * @param {number[]} nums
 * @return {number}
 */
var removeDuplicates = function(nums) {
  for (var i=1;i<nums.length-1;i++) {
    var n1 = nums[i-1];
    var n2 = nums[i];
    var n3 = nums[i+1];
    if (n1 === n2 && n2 === n3) {
      nums.splice(i+1,1);
      i--;
    }
  }
};
