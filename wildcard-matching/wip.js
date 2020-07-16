/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
var isMatch = function(s, p) {
  var si = 0;
  var pi = 0;
  while(true) {
    if (s[si] == null) {
      while (p[pi] === "*") {
        pi++;
      }
      return p.length === pi;
    } else if (p[pi] == null) {
      return false;
    } else if (p[pi] === "*") {
      while (p[pi+1] === "*") {
        pi++;
      }
      return p[pi+1] == null || isMatch(s.substring(si), p.substring(pi+1)) || isMatch(s.substring(si+1), p.substring(pi))
    } else if (p[pi] === "?" || p[pi] === s[si]) {
      pi++;
      si++;
      continue;
    } else {
      return false;
    }
  } 
};
