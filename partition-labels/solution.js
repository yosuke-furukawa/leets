/**
 * @param {string} S
 * @return {number[]}
 */
var partitionLabels = function(S) {
  var last = new Array(26);
  for (var i=0;i<S.length;i++) {
    last[S.charAt(i).charCodeAt()-'a'.charCodeAt()] = i;
  }
  
  var j=0;
  var anchor = 0;
  var ans = [];
  for (var i=0;i<S.length;i++) {
    j = Math.max(j, last[S.charAt(i).charCodeAt()-'a'.charCodeAt()]);
    if (i === j) {
      ans.push(i - anchor + 1);
      anchor = i + 1
    }
  }
  return ans;
};
