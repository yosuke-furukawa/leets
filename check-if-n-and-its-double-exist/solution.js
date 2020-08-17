/**
 * @param {number[]} arr
 * @return {boolean}
 */
var checkIfExist = function(arr) {
  var map = new Map();
  for (var i=0;i<arr.length;i++) {
    map.set(arr[i]/2, i);
  }
  console.log(map);
  for (var i=0;i<arr.length;i++) {
    if (map.has(arr[i]) && map.get(arr[i]) !== i) {
      return true;
    }
  }
  return false;
};
