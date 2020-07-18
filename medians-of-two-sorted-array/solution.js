/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
var findMedianSortedArrays = function(nums1, nums2) {
  var len = nums1.length + nums2.length;
  var pivot = len / 2;
  
  var i = 0, j = 0, n1 = 0, n2 = 0, pre = 0;
  while (pivot >= 0) {
    pre = Math.min(n1, n2);
    n1 = nums1[i] ?? Infinity;
    n2 = nums2[j] ?? Infinity;
    if (n1 < n2) {
      i++;
    } else if (n2 <= n1) {
      j++;
    }
    pivot--;
  }
  return len % 2 === 0 ? (pre + Math.min(n1, n2)) / 2 : Math.min(n1, n2);
};
