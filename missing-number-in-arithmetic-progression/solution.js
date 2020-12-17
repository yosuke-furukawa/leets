/**
 * @param {number[]} arr
 * @return {number}
 */
var missingNumber = function(arr) {
  var diff = Infinity;
  var sign = 0;
  
  for (var i=0;i<arr.length-1;i++) {
    var n1 = arr[i];
    var n2 = arr[i+1];
    if (Math.abs(n2 - n1) < diff) {
      diff = Math.abs(n2 - n1);
      sign = Math.sign(n2 - n1);
    }
  }
  if (diff === 0) {
    return arr[0];  
  }
  
  for (var i=0;i<arr.length-1;i++) {
    var n1 = arr[i];
    var n2 = arr[i+1];
    if (Math.abs(n2 - n1) !== diff) {
      return n1 + sign * diff;
    }
  }
};
