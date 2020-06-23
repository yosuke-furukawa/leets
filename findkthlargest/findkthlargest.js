/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */

var partition = function(nums, left, right, pivotIndex) {
  var pivot = nums[right];
  var storeIndex = left; 
  for (var i = left; i <= right; i++) {
    if(nums[i] < pivot) { 
      [nums[i], nums[storeIndex]] = [nums[storeIndex], nums[i]];
      storeIndex++; 
    } 
  } 

  [nums[right], nums[storeIndex]] = [nums[storeIndex], nums[right]];
  return storeIndex; 
}

var quickselect = function(left, right, nums, k) {
  while(true) {
    if (left === right) {
      return nums[left];
    }
    var pivotIndex = partition(nums, left, right, pivotIndex);
    if (k === pivotIndex) {
      return nums[k]
    } else if (k < pivotIndex) {
      right = pivotIndex - 1;
    } else {
      left = pivotIndex + 1;
    }
  }
}

var findKthLargest = function(nums, k) {
  return quickselect(0, nums.length-1, nums, nums.length - k);
};
