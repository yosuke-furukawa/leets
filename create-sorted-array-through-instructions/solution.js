/**
 * @param {number[]} instructions
 * @return {number}
 */
var createSortedArray = function(instructions) {
  var result = 0;
  var array = [];
  for (var v of instructions) {
    var left = 0;
    var right = array.length;
    var mid = Math.floor((left + right) / 2);;
    while (left < right) {
      // console.log({left, right, mid, v});
      if (v < array[mid]) {
        right = mid;
      } else if (v > array[mid]) {
        left = mid + 1;
      } else {
        var index = array.indexOf(v);
        var lastIndex = array.lastIndexOf(v) + 1;
        if (Math.min(index, array.length - index) < Math.min(lastIndex, array.length - lastIndex)) {
          mid = index;
        } else {
          mid = lastIndex;
        }
        break;
      }
      mid = Math.floor((left + right) / 2);
    }
    // console.log({array, mid, v, start:mid, end: array.length - mid});
    result += Math.min(mid, array.length - mid);
    // console.log({result})
    array.splice(mid, 0, v);
  }
  // console.log(array);
  return result % (10**9+7);
};
