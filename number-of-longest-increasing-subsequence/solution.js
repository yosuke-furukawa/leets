/**
 * @param {number[]} nums
 * @return {number}
 */
var findNumberOfLIS = function(nums) {
  var allMaxLength = 1;
  var dpLength=[];
  var dpCount =[];
  for(var r=0; r<nums.length; r++) {
    var thisValue = nums[r];
    var thisMaxLen = 1;
    var thisMaxCount = 1;
    for(var l=r-1; l>=0; l--) {
      if(nums[l]<thisValue) {
        var thisLen = dpLength[l]+1;
        var thisCount = dpCount[l];
        if(thisLen > thisMaxLen) {
          thisMaxLen = thisLen;
          thisMaxCount = thisCount;
        } else if(thisLen === thisMaxLen) {
          thisMaxCount += thisCount;
        }
      }
    }
    dpLength[r] = thisMaxLen;
    dpCount[r] = thisMaxCount;
    allMaxLength = Math.max(allMaxLength, thisMaxLen);
  }
  // all count
  var allCount = 0;
  for(var i=0; i<dpLength.length; i++) {
    if(dpLength[i] === allMaxLength) {
      allCount += dpCount[i];
    }
  }
  return allCount;
};
