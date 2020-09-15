/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLastWord = function(s) {
  var str = s.split(" ").filter((a) => a.length>0);
  if (str.length === 0) {
    return 0;
  }
  
  return str[str.length-1].length;
};
