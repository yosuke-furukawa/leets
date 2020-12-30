/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var threeSumSmaller = function(nums, target) {  
  var count = 0;
  for (var i=0;i<nums.length;i++) {
    var n1 = nums[i];
    for (var j=i+1;j<nums.length;j++) {
      var n2 = nums[j];
      var array = nums.slice(j+1);
      count += array.filter((n3) => n1+n2+n3 < target).length;
    }
  }
  return count;
};
