/**
 * @param {number[]} nums
 * @return {number}
 */
var minimumMountainRemovals = function(nums) {
  var n = nums.length;
  var left = new Array(n).fill(1);
  var right = new Array(n).fill(1);
  for(var i=1;i<n;i++){
    for(var j=0;j<i;j++){
      if(nums[j] < nums[i] && left[i] < left[j]+1) {
        left[i] = left[j]+1;        
      }
    }
  }

  for(var i=n-2;i>=0;i--){
    for(var j=n-1;j>i;j--){
      if(nums[j]<nums[i]&&right[i]<right[j]+1)
        right[i]=right[j]+1;
    }
  }
        
  var max=0;
  for(var i=1;i<n-1;i++){
    max = Math.max(max,left[i]+right[i]-1);
  }

  return n-max;
};
