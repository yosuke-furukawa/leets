/**
 * @param {string[]} words
 * @param {number} maxWidth
 * @return {string[]}
 */
var fullJustify = function(words, maxWidth) {
  var results = [];
  for (var i=0;i<words.length;i++) {
    var word = words[i];
    var array = [word];
    var count = array.reduce((acc, cur) => acc + cur.length, 0);
    if (count >= maxWidth) {
      results.push(array.join(""));
      continue;
    }
    for (var j=i+1;j<words.length;j++) {
      array.push(words[j]);
      count = array.reduce((acc, cur) => acc + cur.length + 1, 0) - 1;
      // console.log(array, count);
      if (count > maxWidth) {
        array.pop();
        let count = array.reduce((acc, cur) => acc + cur.length, 0);
        var str = "";
        var length = array.length;
        var spaceLeft = maxWidth - count;
        while (array.length > 0) {
          var w = array.shift();
          str += w;
          var spaces = length !== 1 ? Math.ceil((spaceLeft) / (length - 1)) : spaceLeft;
          for (var s=0;s<spaces;s++) {
            str += " ";
          }
          count = array.reduce((acc, cur) => acc + cur.length, 0);
          length = array.length;
          spaceLeft = maxWidth - str.length - count;
        }
        results.push(str);
        i = j-1;
        break;
      } else if (count === maxWidth) {
        results.push(array.join(" "));
        array = [];
        i = j;
        break;
      }
    }
    
    if (array.length > 0) {
      var rest = array.join(" ");
      results.push(rest.padEnd(maxWidth, " "));
      break;
    }
  }
  
  
  return results;
};
