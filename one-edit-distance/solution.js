/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isOneEditDistance = function(s, t) {
  var long = s.length < t.length ? t : s;
  var short = s.length < t.length ? s : t;
  
  if (long.length - short.length > 1) {
    return false;
  }
  
  if (long.length === short.length) {
    var oneEdit = false;
    for (var i=0;i<long.length;i++) {
      if (long[i] !== short[i]) {
        if (oneEdit) {
          return false;
        }
        oneEdit = true;
      }
    }
    return oneEdit;
  }
  
  var si = 0;
  var oneEdit = false;
  for (var li=0;li<long.length;li++) {
    if (long[li] !== short[si]) {
      if (oneEdit) {
        return false;
      }
      oneEdit = true;
      continue;
    }
    si++;
  }
  return true;
};
