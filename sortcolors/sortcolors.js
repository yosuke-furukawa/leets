/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var sortColors = function(nums) {
  var arr = new Array(3).fill(0);
  for (var i=0;i<nums.length;i++) {
    arr[nums[i]]++;
  }
  var index = 0;
  for (var a=0;a<arr.length;a++) {
    var c = arr[a];
    while(c--) {
      nums[index] = a;
      index++;
    }
  }
};
