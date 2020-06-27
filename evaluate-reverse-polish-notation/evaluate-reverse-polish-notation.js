/**
 * @param {string[]} tokens
 * @return {number}
 */

const ops = ["+", "-", "*", "/"];
var evalRPN = function(tokens) {
  var pos = 0;
  while (tokens.length > 1) {
    var token = tokens[pos];
    if (ops.includes(token)) {
      var expressions = tokens.splice(pos - 2, 3);
      var answer;
      switch (token) {
        case "+": 
          answer = Number(expressions[0]) + Number(expressions[1]);
          break;
        case "-": 
          answer = Number(expressions[0]) - Number(expressions[1]);
          break;
        case "*":
          answer = Number(expressions[0]) * Number(expressions[1]);
          break;
        case "/":
          answer = Math.trunc(Number(expressions[0]) / Number(expressions[1]));
          break;
      }
      tokens.splice(pos-2, 0, answer);
      pos = pos - 1;
      continue;
    }
    pos++;
  }
  return tokens[0];
};
