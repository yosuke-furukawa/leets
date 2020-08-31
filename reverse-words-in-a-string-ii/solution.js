/**
 * @param {character[]} s
 * @return {void} Do not return anything, modify s in-place instead.
 */
var reverseWords = function(s) {
  var start = 0;
  for (var i=0;i<s.length;i++) {
    if (s[i] === " " || i === s.length - 1) {
      var end = i;
      if (i === s.length - 1) {
        end = i+1;
      }
      var term = s.slice(start, end);
      s.splice(start, term.length, ...term.reverse());
      start = i+1;
      term = [];
    }
  }
  s.reverse();
};
