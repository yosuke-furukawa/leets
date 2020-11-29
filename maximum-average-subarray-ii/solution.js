/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var findMaxAverage = function(nums, k) {
  var memo = new Array(nums.length - k + 1).fill(0);
  
  var max = -Infinity;

  for (var i=0;i<=nums.length-k;i++) {
    var ns = nums.slice(0, k+i);
    var avg = ns.reduce((acc, cur) => acc+cur) / (k+i);
    memo[i] = avg;
    max = Math.max(max, memo[i]);
  }
  // console.log(memo)
  for (var i=0;i<nums.length-k+1;i++) {
    var temp = new Array(nums.length - k - i).fill(0);
    for (var j=0;j<temp.length;j++) {
      var sum = memo[j+1] * (k+j+1);
      var val = sum - nums[i];
      temp[j] = val / (k+j);
      max = Math.max(max, temp[j]);
    }
    memo = temp;
  }

  return max;
};
