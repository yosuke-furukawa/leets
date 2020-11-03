/**
 * @param {string} s
 * @return {number}
 */
var maxPower = function(s) {
  var arr = new Array(s.length).fill(1);
  for (var i=0;i<s.length;i++) {
    var c = s[i];
    var p = s[i-1];
    if (c === p) {
      arr[i] = arr[i-1]+1; 
    }
  }
  return Math.max(...arr);
};
