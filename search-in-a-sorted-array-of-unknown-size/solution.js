/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * function ArrayReader() {
 *
 *     @param {number} index
 *     @return {number}
 *     this.get = function(index) {
 *         ...
 *     };
 * };
 */

/**
 * @param {ArrayReader} reader
 * @param {number} target
 * @return {number}
 */
var search = function (reader, target) {
  var left = 0, right = 10 ** 4;
  mid = left + Math.floor((right - left) / 2);
  while (left + 1 < right){
    mid = left + Math.floor((right - left) / 2);
    var x = reader.get(mid);
    if (x === target) {
      return mid;
    } else if (x < target) {
      left = mid;
    } else {
      right = mid;
    }
  }

  if(reader.get(left) === target) {
    mid = left;
    return mid;
  }
  if(reader.get(right) === target) {
    mid = right;
    return mid;
  }
  return -1;

};
