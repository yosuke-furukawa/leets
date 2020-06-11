var romanum = {
  I: 1,
  V: 5,
  X: 10,
  L: 50,
  C: 100,
  D: 500,
  M: 1000,
};

var romanToInt = function(s) {
  var result = 0;
  for (var i=0; i<s.length;i++) {
    var n;
    if (romanum[s[i]] < romanum[s[i+1]]) {
      n = -romanum[s[i]];
    } else {
      n = romanum[s[i]];
    }
    result += n;
  }

  return result;
}

console.log(romanToInt("XVI"));
console.log(romanToInt("IV"));
console.log(romanToInt("LVIII"));
console.log(romanToInt("MCMXCIV"));
