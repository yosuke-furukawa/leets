/**
 * @param {string} s
 * @return {string[]}
 */
function check(s) {
  var left = 0, right = 0;
  for (var i=0;i<s.length;i++) {
    if (s[i] === "(") {
      left++;
    } else if (s[i] === ")") {
      if (left > 0) {
        left--;
      } else {
        right++;
      }
    }
  }
  return left === 0 && right === 0;
}

var removeInvalidParentheses = function(s) {
  if (!s.includes("(") && !s.includes(")")) {
    return [s];
  }
  var left = 0, right = 0;
  for (var i=0;i<s.length;i++) {
    if (s[i] === "(") {
      left++;
    } else if (s[i] === ")") {
      if (left > 0) {
        left--;
      } else {
        right++;
      }
    }
  }
  
  if (left === 0 && right === 0) {
    return [s]
  }
  var set = new Set();
  var visited = new Set();
  var removes = (s, left, right) => {
    if (visited.has(s)) {
      return null;
    }
    visited.add(s);
    var result = String(s);
    if (right === 0 && left === 0) {
      return result;
    }
    if (right > 0) {
      for (var i=0;i<result.length;i++) {
        if (result[i] === ")") {
          var next = result.substring(0, i) + result.substring(i+1, result.length);
          var r = removes(next, left, right-1);
          r && set.add(r);
        }
      }
    }
  
    if (left > 0) {
      for (var i=result.length-1;i>=0;i--) {
        if (result[i] === "(") {
          var next = result.substring(0, i) + result.substring(i+1, result.length);
          var l = removes(next, left-1, right);
          l && set.add(l);
        }
      }
    }
  }
  removes(s, left, right);
  var temp = [...set];
  var results = [];
  for (const r of temp) {
    if (check(r)) {
      results.push(r);
    };
  }
  
  if (results.length === 0) {
    return [""];
  }
  return results;
};
