/**
 * @param {number[][]} arrays
 * @return {number}
 */
var maxDistance = function(arrays) {
  var min1 = Infinity;
  var max1 = -Infinity;
  var max1Index = 0;
  var diff1 = 0;
  // max first
  for (var i=0;i<arrays.length;i++) {
    if (max1 < arrays[i][arrays[i].length - 1]) {
      max1 = arrays[i][arrays[i].length - 1];
      max1Index = i;
    }
  }
  
  for (var i=0;i<arrays.length;i++) {
    if (max1Index !== i && min1 > arrays[i][0]) {
      min1 = arrays[i][0];
    }
  }
  diff1 = Math.abs(max1 - min1);
  
  // min first
  var min2 = Infinity;
  var max2 = -Infinity;
  var min2Index = 0;
  var diff2 = 0;
  
  for (var i=0;i<arrays.length;i++) {
    if (min2 > arrays[i][0]) {
      min2 = arrays[i][0];
      min2Index = i;
    }
  }
  for (var i=0;i<arrays.length;i++) {
    if (min2Index !== i && max2 < arrays[i][arrays[i].length - 1]) {
      max2 = arrays[i][arrays[i].length - 1];
    }
  }
  diff2 = Math.abs(max2 - min2);
  return Math.max(diff1, diff2);
};
