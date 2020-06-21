/**
 * @param {number} n
 * @return {string[]}
 */

var generateParenthesis = function(n) {
  var answer = [];
  if (n==0) {
    answer.push("");
  } else {
    for (var c=0;c<n;c++){
      for (var left of generateParenthesis(c)) {
        for (var right of generateParenthesis(n-1-c)) {
          answer.push("(" + left + ")" + right);
        }
      }
    }
  }
  return answer;
};
