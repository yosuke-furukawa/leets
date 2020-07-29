/**
 * @param {string} S
 * @param {number} K
 * @return {string}
 */
var licenseKeyFormatting = function(S, K) {
  var str = S.split("-").join("");
  var result = "";
  for (var i=str.length-1;i>=0;i--) {
    var s = str[i];
    
    if (i !== 0 && (str.length-i)%K === 0) {
      s = "-" + s;
    }
    result = s + result;
  }
  return result.toUpperCase();
};
