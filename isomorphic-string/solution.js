/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isIsomorphic = function(s, t) {
  var sa = new Array(s.length);
  var ta = new Array(t.length);
  if (s.length != t.length) {
    return false;
  }
  
  var smap = new Map();
  var tmap = new Map();
  for (var i=0;i<sa.length;i++) {
    var sc = s[i];
    var tc = t[i];
    if (smap.get(sc) !== tmap.get(tc)) {
      return false;
    }
    
    if (!smap.has(sc)) {
      sa[i] = sc;
      smap.set(sc, i);
    }
    if (!tmap.has(tc)) {
      ta[i] = tc;
      tmap.set(tc, i);
    }
  }
  return true;
};
