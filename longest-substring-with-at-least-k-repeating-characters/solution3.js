/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
var longestSubstring = function(s, k) {
  var str = [...s];
  var countMap = new Array(26);
  var maxUnique = getMaxUniqueLetters(s);
  var result = 0;
  
  for (var currUnique=1;currUnique<=maxUnique;currUnique++) {
    countMap.fill(0);
    var windowStart = 0, windowEnd = 0, idx = 0, unique = 0, countAtLeastK = 0;
    while (windowEnd < str.length) {
      if (unique <= currUnique) {
        idx = str[windowEnd].charCodeAt() - 97;
        if (countMap[idx] === 0) {
          unique++;
        }
        countMap[idx]++;
        if (countMap[idx] === k) {
          countAtLeastK++;
        }
        windowEnd++;
      } else {
        idx = str[windowStart].charCodeAt() - 97;
        if (countMap[idx] === k) {
          countAtLeastK--;
        }
        countMap[idx]--;
        if (countMap[idx] === 0) {
          unique--;
        }
        windowStart++;
      }
      
      if (unique === currUnique && unique === countAtLeastK) {
        result = Math.max(windowEnd - windowStart, result);        
      }
    }
  }
  
  return result;
};

function getMaxUniqueLetters(s) {
  var set = new Set();
  for (var i=0;i<s.length;i++) {
    if (!set.has(s[i])) {
      set.add(s[i]);
    }
  }
  return set.size;
}
