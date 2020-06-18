/**
 * @param {number[]} arr
 * @return {number}
 */
var countElements = function(arr) {
    var map = new Map();
    for (var i = 0; i<arr.length; i++) {
        map.set(arr[i], true);
    }
    var count = 0;
    for (var i = 0; i<arr.length; i++) {
        if (map.has(arr[i]+1)) {
            count++;
        }
    }
    return count;
};
