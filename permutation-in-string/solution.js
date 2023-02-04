/**
 * @param {string} s1
 * @param {string} s2
 * @return {boolean}
 */
var checkInclusion = function(s1, s2) {
  const map = new Map();
  for (const s of s1) {
    if (map.has(s)) {
      map.set(s, map.get(s)+1);
    } else {
      map.set(s, 1);
    }
  }
  
  for (var i=0;i<=s2.length-s1.length;i++) {
    var s2substr = s2.substring(i, i+s1.length);
    const m = new Map(map);
    
    for (const t of s2substr) {
      if (!m.has(t)) {
        break;
      }
      m.set(t, m.get(t)-1);
      if (m.get(t) === 0) {
        m.delete(t);
      }
    }
    if (m.size === 0) {
      return true;
    }
  }
  return false;
};
