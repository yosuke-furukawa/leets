/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var wiggleSort = function(nums) {
  nums.sort((a, b) => a-b);
  var n = 0;
  var pivot = Math.floor((nums.length - 1) / 2);
  while (pivot >= 0) {
    var [a] = nums.splice(pivot, 1);
   
    if (nums.length -1 -2*n < 0) {
      nums.push(a);
      break;
    }
    var [b] = nums.splice(nums.length-1 - 2*n, 1);
    nums.push(a, b);
    pivot -= 1;
    n++;
  }
};
