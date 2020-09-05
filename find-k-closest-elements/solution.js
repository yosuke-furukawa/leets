/**
 * @param {number[]} arr
 * @param {number} k
 * @param {number} x
 * @return {number[]}
 */
var findClosestElements = function(arr, k, x) {
  if (arr.length == 0) {
    return arr;
  }
  var left = 0, right = arr.length - 1;
  mid = left + Math.floor((right - left) / 2);
  while (left + 1 < right){
    mid = left + Math.floor((right - left) / 2);
    if (arr[mid] === x) {
      break;
    } else if (arr[mid] < x) {
      left = mid;
    } else {
      right = mid;
    }
  }

  if(arr[left] === x) {
    mid = left;
  }
  if(arr[right] === x) {
    mid = right;
  }
  
  if (Math.abs(x - arr[mid-1]) <= Math.abs(x - arr[mid])) {
    mid = mid-1;
  }
  var m = arr[mid];
  
  var list = [];
  if (m != null) {
    k--;
    list.push(m);
  }
  var lefts = arr.slice(0, mid);
  var rights = arr.slice(mid + 1);
  
  while (k > 0) {
    // console.log({lefts, rights});
    var l = Infinity;
    if (lefts.length > 0) {
      l = lefts.pop();  
    }
    var r = Infinity;
    if (rights.length > 0) {
      r = rights.shift();
    }
    
    if (Math.abs(l-x) <= Math.abs(r-x)) {
      r != Infinity && rights.unshift(r);
      list.unshift(l);
    } else {
      l != Infinity && lefts.push(l);
      list.push(r);
    }
    k--;
  }
  return list;
};
