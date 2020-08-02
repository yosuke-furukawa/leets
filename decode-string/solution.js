/**
 * @param {string} s
 * @return {string}
 */
var decodeString = function(s) {
  var captureNumber = (s, j) => {
    var num = "";
    var i = j;
    for  (;i<s.length;i++) {
      if (s[i] == +s[i]) {
        num += s[i];
      } else {
        break;
      }
    }
    return [num, i];
  }
  var getNextParen = (s, i) => {
    var parenCount = 0;
    for (var index=i;index<s.length;index++) {
      var char = s[index];
      if (char === "[") {
        parenCount++;
      } else if (char === "]") {
        parenCount--;
        if (parenCount === 0) {
          return index;
        }
      }
    }
  }
  var decode = (s) => {
    if (!s.includes("[")) {
      return s;
    }
    for (var i=0;i<s.length;i++) {
      var c = s[i];
      if (c == +c) {
        var numbegin = i;
        
        var [num, index] = captureNumber(s, numbegin);
        var nextParen = getNextParen(s, index);
        var chars = s.substring(index+1, nextParen);
        var pre = s.substring(0, numbegin);
        var target = chars.repeat(+num);
        var post = s.substring(nextParen+1);
        s = pre + target + post;
        i = -1;
      }
    }
    return decode(s);
  }
  return decode(s);
};
