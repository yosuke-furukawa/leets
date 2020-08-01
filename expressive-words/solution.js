/**
 * @param {string} S
 * @param {string[]} words
 * @return {number}
 */
var expressiveWords = function(S, words) {
  var str = [...S];
  var mappings = "";

  for (var i=0;i<str.length;i++) {
    var s1 = str[i];
    mappings += s1;
    var count = 1;
    var j = i+1;
    var hasPlus = false;
    while (j < str.length) {
      var s2 = str[j];
      if (s1 !== s2) {
        if (hasPlus) {
          mappings += `{1,${count}}`;
        }
        break;
      }
      count++;
      if (count == 3) {
        hasPlus = true;
      }
      if (hasPlus) {
        i = j;
      }
      j++;
      if (j === str.length) {
        if (hasPlus) {
          mappings += `{1,${count}}`;
        }
      }
    }
  }
  
  const regex = new RegExp("^" + mappings + "$", "g");
  return words.filter((word) => Boolean(word.match(regex))).length;
};
