/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
var longestSubstring = function(s, k) {
  var helper = (s, start, end, k) => {
    if (end < k) return 0;
    
    var countMap = new Array(26).fill(0);
    
    for (var i=start;i<end;i++) {
      countMap[s[i].charCodeAt()-97]++;
    }
    for (var mid=start;mid<end;mid++) {
      if (countMap[s[mid].charCodeAt() - 97] >= k) {
        continue;
      }
      
      var midNext = mid + 1;
      while (midNext < end && countMap[s[midNext] - 97] < k) {
        midNext++;
      }
      
      return Math.max(helper(s, start, mid, k), helper(s, midNext, end, k));
    }
    
    return (end - start);
  };
  return helper(s, 0, s.length, k);
};
