/**
 * @param {number[]} nums
 * @param {number} threshold
 * @return {number}
 */
var smallestDivisor = function(nums, threshold) {
  var sum = nums.reduce((acc, cur) => acc+cur);
  var min = Math.ceil(sum/threshold);
  if (threshold - nums.length === 0) {
    return min;
  }
  var max = Math.ceil(sum/(threshold-nums.length));
  var mid = Math.floor((max + min) / 2);
  var pres;
  while (min <= max) {
    mid = Math.ceil((max + min) / 2);
    // console.log(min, mid, max);
    var s = nums.reduce((acc, cur) => acc + Math.ceil(cur/mid), 0);
    // console.log({s, d})
    if (s > threshold) {
      min = mid + 1;
    } else {
      max = mid - 1;
    }
  }
  if (s <= threshold) {
    return mid;
  } else {
    return mid+1;    
  }
};
