/**
 * @param {string} s
 * @param {string} p
 * @return {number[]}
 */
var findAnagrams = function(s, p) {
  var result = [];
  var sps = [...p];
  var map = new Map();
  
  for (var sp of sps) {
    if (map.has(sp)) {
      map.set(sp, map.get(sp)+1);
      continue;
    }
    map.set(sp, 1);
  }
  
  for (var i=0;i<s.length-p.length+1;i++) {
    var xs = s.substring(i, i+p.length).split("");
    var newmap = new Map(map);
    for (var x of xs) {
      if (!newmap.has(x)) {
        break;
      }
      newmap.set(x, newmap.get(x)-1);
      if (newmap.get(x) <= 0) {
        // console.log("delete");
        newmap.delete(x);
              // console.log(newmap);
      }
    }
    // console.log(newmap);
    if (newmap.size === 0) {
      result.push(i);
    }
    if (!map.has(s[i+p.length-1])) {
      i += p.length-1;
    }
  }
  return result;
};
