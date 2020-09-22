/**
 * @param {number[]} nums
 * @return {number[]}
 */
var majorityElement = function(nums) {
  var count1 = 0;
  var count2 = 0;
  
  var candidate1 = null;
  var candidate2 = null;
  
  for (var n of nums) {
    if (candidate1 != null && candidate1 === n) {
      count1++;
    } else if (candidate2 != null && candidate2 === n) {
      count2++;
    } else if (count1 === 0) {
      candidate1 = n;
      count1++;
    } else if (count2 === 0) {
      candidate2 = n;
      count2++;
    } else {
      count1--;
      count2--;
    }
  }
  
  var result = [];
  count1 = 0;
  count2 = 0;
  
  for (var n of nums) {
    if (candidate1 != null && n === candidate1) {
      count1++;
    }
    if (candidate2 != null && n === candidate2) {
      count2++;
    }
  }
  
  var n = nums.length;
  
  if (count1 > n/3) {
    result.push(candidate1);
  }
  if (count2 > n/3) {
    result.push(candidate2);
  }
  
  return result;
};
