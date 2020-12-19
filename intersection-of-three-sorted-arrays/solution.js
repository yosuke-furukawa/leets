/**
 * @param {number[]} arr1
 * @param {number[]} arr2
 * @param {number[]} arr3
 * @return {number[]}
 */
var arraysIntersection = function(arr1, arr2, arr3) {
  var i = 0;
  var j = 0;
  var k = 0;
  var result = [];
  while (i < arr1.length || j < arr2.length || k< arr3.length) {
    var a1 = arr1[i] ?? Infinity;
    var a2 = arr2[j] ?? Infinity;
    var a3 = arr3[k] ?? Infinity;
    if (a1 === a2 && a2 === a3) {
      result.push(a1);
      i++;
      j++;
      k++;
      continue;
    }
    
    var max = Math.max(a1, a2, a3);
    if (a1 < max) {
      i++;
    }
    if (a2 < max) {
      j++;
    }
    if (a3 < max) {
      k++;
    }
  }
  
  return result;
};
