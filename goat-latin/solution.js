/**
 * @param {string} S
 * @return {string}
 */
const vowels = ["a", "i", "u", "e", "o", "A", "I", "U", "E", "O"];

var toGoatLatin = function(S) {
  if (S.length === 0) {
    return S;
  }
  var arr = S.split(" ");
  for (var i=0;i<arr.length;i++) {
    var chars = [...arr[i]];
    if (vowels.includes(chars[0])) {
      chars.push("ma");
    } else {
      chars.push(chars.shift());
      chars.push("ma");
    }
    
    for (var j=0;j<i+1;j++) {
      chars.push("a");
    }
    arr[i] = chars.join("");
  }
  return arr.join(" ");
};
