/**
 * @param {string} s
 * @return {boolean}
 */
var repeatedSubstringPattern = function(s) {
  if (s.length === 0) {
    return false;
  }

  var original = [...s];
  for (var i=2;i<=s.length;i++) {
    if (s.length % i != 0) {
      continue;
    }
    var fp = 0;
    var mid = 0;
    var strs = [];
    while (fp < s.length) {
      mid += s.length / i;
      var str = original.slice(fp, mid).join("");
      strs.push(str);
      fp = mid;
    }
    if (strs.length < 2) {
      continue;
    }
    var isAllSame = true;
    for (var n=0;n<strs.length-1;n++) {
      var str1 = strs[n];
      var str2 = strs[n+1];
      if (str1 !== str2) {
        isAllSame = false;
        break;
      }
    }
    if (isAllSame) {
      return true;
    }
  }
  
  return false;
};
