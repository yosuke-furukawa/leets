/**
 * @param {string[][]} equations
 * @param {number[]} values
 * @param {string[][]} queries
 * @return {number[]}
 */
var calcEquation = function(equations, values, queries) {
  // key = a/b value = 3.0
  // a/b * c/a
  const variableMap = new Map();
  const divMap = new Map();
  
  for (var i=0;i<equations.length;i++) {
    var [dividiend, divisor] = equations[i];
    var value = values[i];
    variableMap.set(`${dividiend}/${divisor}`, value);
    variableMap.set(`${divisor}/${dividiend}`, 1/value);
    
    if (divMap.has(dividiend)) {
      divMap.get(dividiend).push(divisor);
    } else {
      divMap.set(dividiend, [divisor]);
    }
    
    if (divMap.has(divisor)) {
      divMap.get(divisor).push(dividiend);
    } else {
      divMap.set(divisor, [dividiend]);
    }
  }
  
  var results = [];
  for (const [dividiend, divisor] of queries) {
    
    var ans = variableMap.get(`${dividiend}/${divisor}`);
    if (ans != null) {
      results.push(ans);
      continue;
    }
    
    if (!divMap.has(dividiend)) {
      results.push(-1);
      continue;
    }
    
    if (!divMap.has(divisor)) {
      results.push(-1);
      continue;
    }
    
    if (dividiend === divisor) {
      results.push(1.0);
      continue;
    }
    
    var vars = [dividiend];
    var visited = new Set();
    var v;
    while (vars.length > 0) {
      v = vars[vars.length-1];
      visited.add(v);
      if (v === divisor) {
        var paths = vars;
        var o = paths[0];
        var value = 1;
        for (var p=1;p<paths.length;p++) {
          var d = paths[p];
          var ans = variableMap.get(`${o}/${d}`);
          if (ans == null) {
            continue;
          }
          value = ans * value;
          if (!variableMap.has(`${dividiend}/${d}`)) {
            variableMap.set(`${d}/${dividiend}`, 1/value);
            variableMap.set(`${dividiend}/${d}`, value);
          }
          o = d;
        }
        results.push(value);
        break;
      }
      var temps = divMap.get(v);
      temp = temps.filter((t) => !visited.has(t))[0];
      if (temp != null) {
        vars.push(temp);        
      } else {
        vars.pop();
      }
    }
    if (vars.length === 0) {
      results.push(-1.0);
    }
  }
  
  return results;
};
