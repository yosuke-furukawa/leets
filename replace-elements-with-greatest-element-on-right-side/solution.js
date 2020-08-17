/**
 * @param {number[]} arr
 * @return {number[]}
 */
var replaceElements = function(arr) {
  var max = -1;
  var results = new Array(arr.length);
  for (var i=arr.length-1;i>=0;i--) {
    max = Math.max(max, ...arr.slice(i+1));
    results[i] = max;
  }
  return results;
};
