/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */

var minWindow = function(s, t) {
  var left = 0;
  var right = 0;
  var tfreq = new Map();
  
  for (var c of t) {
    var count = tfreq.get(c) || 0;
    tfreq.set(c, count+1);
  }
  var required = tfreq.size;
  var uniquechars = 0;
  var sfreq = new Map();
  var alen = -1;
  var aleft = 0;
  var aright = 0;
  while(right < s.length) {
    const char = s[right];
    var count = sfreq.get(char) || 0;
    sfreq.set(char, count+1);
    
    if (tfreq.has(char) && tfreq.get(char) === sfreq.get(char)) {
      uniquechars++;
    }
    
    while (left <= right && uniquechars === required) {
      const char = s[left];
      if (alen === -1 ||  right - left + 1 < alen) {
        alen = right - left + 1;
        aleft = left;
        aright = right;
      }
      
      const count = sfreq.get(char);
      sfreq.set(char, count-1);
      if (tfreq.has(char) && sfreq.get(char) < tfreq.get(char)) {
        uniquechars--;
      }
      left++;
    }
    
    right++;
  }
  
  return alen == -1 ? "" : s.substring(aleft, aright + 1);
};
