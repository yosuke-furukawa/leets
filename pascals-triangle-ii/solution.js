/**
 * @param {number} rowIndex
 * @return {number[]}
 */
var f = [1, 1];

function factorial(n) {
  if (f[n] != null) {
    return f[n];
  }
  var result = f[f.length-1];
  for (var i = f.length;i <= n; i++) {
    result = i * result;
    f.push(result);
  }
  return result;
}

function combination(a, b) {
  return factorial(a) / (factorial(b) * factorial(a-b));
}

var getRow = function(rowIndex) {
  // 3 C 0 + 3 C 1 + 3 C 2 + 3 C 3
  var result = [];
  for (var i = rowIndex; i>=0; i--) {
    result.push(combination(rowIndex, i));
  }
  return result;
};
