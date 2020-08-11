/**
 * @param {string} s
 * @return {string}
 */
var makeGood = function(s) {
  var chars = [...s];
  var results = [];
  for (var c=0;c<chars.length;c++){
    var ch = chars[c];
    var pre = chars[c-1] ?? "";
    if (pre === ch) {
      results.push(ch);
    } else if (pre.toUpperCase() !== ch && pre !== ch.toUpperCase()) {
      results.push(ch);
    } else {
      results.splice(c-1, 1);
      chars = [...results, ...chars.slice(c+1)];
      c = 0;
      results = [chars[c]];
    }
  }
  
  return results.join("");
};
