/**
 * @param {string} s
 * @return {number}
 */

var base = "A".charCodeAt();
var end = "Z".charCodeAt();

var map = {};
for (var i=base; i<=end; i++) {
  map[String.fromCharCode(i)] = i - base + 1;
}

var titleToNumber = function(s) {
  var num = 0;
  for (var index=0;index<s.length;index++) {
    var title = s[s.length-1 - index];
    num += map[title] * (26**(index))
  }
  return num;
};
