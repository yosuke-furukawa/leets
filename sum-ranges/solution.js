/**
 * @param {number[]} nums
 * @return {string[]}
 */
var summaryRanges = function(nums) {
  var result = [];
  for (var i=0;i<nums.length;i++) {
    var x = nums[i];
    var temp = x;
    for (var j=i+1;j<nums.length+1;j++) {
      var y = nums[j];
      if (temp + 1 !== y) {
        if (temp === x) {
          result.push(`${x}`);
        } else {
          result.push(`${x}->${temp}`);
          i = j-1;
        }
        break;
      } else if (y == null) {
        result.push(`${x}->${temp}`);
        break;
      } else {
        temp = y;
      }
    }
  }
  console.log(result);
  return result;
};
