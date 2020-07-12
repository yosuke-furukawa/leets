/**
 * @param {number[]} nums
 * @return {number[]}
 */


var countSmaller = function(nums) {
  if (nums.length == 0) {
    return [];
  }
  var result = [0];
  var arr = [nums[nums.length-1]];
  for (var n=nums.length-2;n>=0;n--) {
    var num = nums[n];
    var low = 0;
    var high = arr.length;
    var mid = Math.floor((high + low)/2);
    while(low <= high) {
      if (num < arr[mid]) {
        high = mid - 1;
      } else if (num > arr[mid]) {
        low = mid + 1;
      } else {
        high = high-1;
      }
      mid = Math.floor((low + high)/2);
    }
    if (low > high) {
      arr.splice(mid+1, 0, num);
      result.unshift(mid+1);
    } else {
      arr.splice(mid, 0, num);
      result.unshift(mid);
    }
  }
  return result;
};
