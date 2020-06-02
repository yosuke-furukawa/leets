var firstUniqChar = function(s) {
  for(var i=0; i<s.length; i++) {
    if (s.indexOf(s[i])===s.lastIndexOf(s[i])) {
      return i;
    }
  }
  return -1;
};
