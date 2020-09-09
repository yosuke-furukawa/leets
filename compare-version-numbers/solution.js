/**
 * @param {string} version1
 * @param {string} version2
 * @return {number}
 */
var compareVersion = function(version1, version2) {
  var v1s = version1.split(".");
  var v2s = version2.split(".");
  var len = Math.max(v1s.length, v2s.length);
  for (var i=0;i<len;i++) {
    var v1 = Number.parseInt(v1s[i] ?? 0);
    var v2 = Number.parseInt(v2s[i] ?? 0);
    if (v1 === v2) {
      continue;
    }
    
    if (v1 < v2) {
      return -1;
    } else {
      return 1;
    }
  }
  return 0;
};
