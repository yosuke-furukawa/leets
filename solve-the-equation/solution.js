/**
 * @param {string} equation
 * @return {string}
 */
var solveEquation = function(equation) {
  var eqs = equation.split("=");
  var leftEquation = eqs[0];
  var rightEquation = eqs[1];
  
  var equations = [...tokenize(leftEquation), {symbol: "+", type: "op" }, ...tokenize(rightEquation, true)];
  // console.log(equations);
  var top = equations[0];
  var result = { x:0, num: 0 };
  if (top.type === "x") {
    var s = top.symbol.replace("x", "");
    if (s === "" || s === "-") {
      s = s + "1";
    }
    result.x = +s;
  } else {
    result.num += +top.symbol;
  }
  for (var i=1;i<equations.length-1;i++) {
    // console.log(result);
    var op = equations[i];
    var eq = equations[i+1];
    // console.log({op, eq});
    if (eq.type === "x") {
      var s = eq.symbol.replace("x", "");
      if (s === "" || s === "-") {
        s = s + "1";
      }
      if (op.symbol === "+") {
        result.x += +s;        
      } else {
        result.x -= +s;
      }
    } else if (eq.type === "number") {
      if (op.symbol === "+") {
        result.num += +eq.symbol;
      } else {
        // console.log(eq.symbol, result.num);
        result.num -= +eq.symbol;
      }
    }
  }
  // console.log(result);
  if (result.x === 0 && result.num === 0) {
    return "Infinite solutions";
  }
  if (result.x === 0 && result.num !== 0) {
    return "No solution";
  }
  var res = -1 * result.num / result.x; 
  return `x=${res}`;
};

function tokenize(str, minus = false) {
  var regs = /^-\d*x|\d*x|^-\d+|\d+/g;
  var matches = Array.from(str.matchAll(regs));
  var pos = 0;
  var result = [];
  while (pos < str.length) {
    var char = str[pos];
    if (matches[0].index === pos) {
      var symbol = matches[0][0];
      if (minus) {
        symbol = symbol[0] === "-" ? symbol.replace("-", "") : "-" + symbol;
      }
      if (symbol.indexOf("x") >= 0) {
        result.push({ symbol, type: "x" });
      } else {
        result.push({ symbol, type: "number" });
      }
      pos += matches[0][0].length-1;
      matches.shift();      
    } else if (char === "+" || char === "-") {
      result.push({symbol: char, type: "op" });
    }
    pos++;
  }
  return result;
}
