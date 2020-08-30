/**
 * @param {string} s
 * @return {string}
 */
var reverseWords = function(s) {
  return s.split(" ").map((r) => [...r].reverse().join("")).join(" ");
};
