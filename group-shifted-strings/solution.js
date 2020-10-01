/**
 * @param {string[]} strings
 * @return {string[][]}
 */
var groupStrings = function(strings) {
  var map = new Map();
  for (var str of strings) {
    var k = createKey(str);
    // console.log(k);
    if (map.has(k)) {
      map.get(k).push(str);
      continue;
    }
    map.set(k, [str]);
  }
  
  var result = [];
  for (var key of map.keys()) {
    result.push(map.get(key));
  }
  return result;
};

function mod(x, y) {
  return x - y * Math.floor(x / y);
}

function createKey(str) {
  var key = `${str.length}`;
  for (var i=0;i<str.length-1;i++) {
    var c1 = str[i].charCodeAt()-97;
    var c2 = str[i+1].charCodeAt()-97;
    var diff = c2 - c1;
    if (diff < 0) {
      diff = mod(diff, 26);
    }
    key += diff;
  }
  return key;
}
