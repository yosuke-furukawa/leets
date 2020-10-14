/**
 * @param {number[]} nums
 * @return {number}
 */
var rob = function(nums) {
  var len = nums.length;
  if (len === 0) return 0;
  if (len === 1) return nums[0];
  if (len === 2) return Math.max(nums[0], nums[1]);
  if (len === 3) return Math.max(nums[0], nums[1], nums[2]);

  var dp = (arr) => {
    var memo = [arr[0],Math.max(arr[0],arr[1])];
    for (var i=2;i<len-1;i++){
      memo[i] = Math.max(memo[i-1], memo[i-2]+arr[i]);
    }
    return memo[memo.length-1];
  };

  return Math.max(dp(nums.slice(0, len-1)), dp(nums.slice(1,len)));
};
