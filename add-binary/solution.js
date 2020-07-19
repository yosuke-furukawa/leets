/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
var addBinary = function(a, b) {
  var len = Math.max(a.length, b.length);
  if (a.length < b.length) {
    a = a.padStart(len, "0");
  } else {
    b = b.padStart(len, "0");
  }
  var result = "";
  var carry = "0";
  for (var i=len-1;i>=0;i--) {
    var c = a.charAt(i);
    var d = b.charAt(i);
    if (c === d && c === "0") {
      result = carry + result;
      carry = "0";
    } else if (c === d && c === "1") {
      result = carry + result;
      carry = "1";
    } else if (c !== d) {
      if (carry === "1") {
        result = "0" + result;
        carry = "1";
      } else {
        result = "1" + result;
        carry = "0";
      }
    }
  }
  if (carry === "1") {
    result = carry + result;
  }
  return result;
};
