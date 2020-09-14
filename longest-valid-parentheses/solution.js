/**
 * @param {string} s
 * @return {number}
 */
var longestValidParentheses = function(s) {
  const stock = [-1];
  let max = 0;
  
  for (let i=0;i<s.length;i++) {
    const top = stock[stock.length-1];
    if (s[top] === '(' && s[i] === ')') {
      stock.pop();
      const newTop = stock[stock.length-1];
      max = Math.max(i - newTop, max);
    } else {
      stock.push(i);
    }
  }
  return max;
};
