var longestCommonPrefix = function(strs) {
  if (strs.length === 0) {
    return "";
  }
  var minstr = strs[0];
  for (var i=1; i< strs.length;i++) {
    var str = strs[i];
    if (minstr.length > str.length) {
      minstr = str;
    }
  }

  for (var m = minstr.length; m >= 0; m--) {
    var pre = minstr.substr(0, m);
    if (strs.every((str) => str.startsWith(pre))) {
      return pre;
    }
  }
  return "";
};

console.log(longestCommonPrefix(["flower","flow","flight"]));
console.log(longestCommonPrefix(["dog","racecar","car"]));
console.log(longestCommonPrefix(["dog","racecar","car"]));
