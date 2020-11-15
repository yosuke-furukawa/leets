
/**
 * @param {number[][]} envelopes
 * @return {number}
 */
var maxEnvelopes = function(envelopes) {
  var sorted = envelopes.sort((a,b) => {
    var diff1 = a[0] - b[0];
    var diff2 = b[1] - a[1];
    if (diff1 === 0) {
      return diff2;
    }
    return diff1;
  });
  var secondDim = sorted.map((e) => e[1]);
  // console.log(secondDim);
  var dp = [];
  var len = 0;
  for (var n of secondDim) {
    var i = search(dp, n);
    if (i < 0) {
      dp.push(n);
    } else {
      dp[i] = n;
    }    
  }
  return dp.length;
};

var search = function(nums, target, left = 0, right = nums.length-1) {
  // console.log(nums)
  var ans = -1;
  while (left <= right) {
    var middle = Math.floor((right + left) /2);
    var m = nums[middle];
    if (m < target) {
      left = middle+1;
    } else {
      ans = middle;
      right = middle-1;
    }
  }
  return ans;
};
