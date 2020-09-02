/**
 * @param {number[]} nums
 * @param {number} k
 * @param {number} t
 * @return {boolean}
 */
var containsNearbyAlmostDuplicate = function(nums, k, t) {
  var arr = nums.map((t, k) => ({t, k}));
  arr = arr.sort((a, b) => a.t - b.t);
  for (var i=0;i<arr.length-1;i++) {
    var num1 = arr[i];
    for (var j=i+1;j<arr.length;j++) {
      var num2 = arr[j];
      var kdiff = Math.abs(num1.k - num2.k);
      var tdiff = Math.abs(num1.t - num2.t);
      // console.log(num1, num2, tdiff, kdiff)
      if (tdiff <= t && kdiff <= k) {
        return true;
      }
      
      if (tdiff > t && kdiff > k) {
        break;
      }
    }
  }
  return false;
};
