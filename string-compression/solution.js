/**
 * @param {character[]} chars
 * @return {number}
 */
var compress = function(chars) {
  if (chars.length === 0) {
    return 0;
  }
  
  for (var i=0;i<chars.length;) {
    var c1 = chars[i];
    var count = 1;
    for (var j=i+1;j<chars.length;j++) {
      var c2 = chars[j];
      if (c1 === c2) {
        count++;
      } else {
        break;
      }
    }
    if (count > 1) {
      var str = [...(count+"")];
      chars.splice(i+1, count-1, ...str);
      i += str.length + 1;
      continue;
    }
    i++;
    // console.log(chars);
  }
  return chars.length;
};
