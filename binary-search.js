function binarySearch(arr, x) {
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
  return arr[mid];
}
