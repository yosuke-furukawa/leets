/**
 * @param {string} S
 * @param {number} K
 * @return {string}
 */
var decodeAtIndex = function(S, K) {
  var strings = S.split(/(\d+)/).filter(Boolean);
  var last = strings[strings.length-1];
  if (last != +last) {
    strings.push("1");
  }
  var decodes = [];
  var obj = {};
  for (var s of strings) {
    if (s == +s) {
      var d = [...s].map(Number).reduce((acc, cur) => acc * cur, 1);
      obj["repeats"] = d;
      decodes.push({...obj});
      obj = {};
    } else {
      obj["str"] = s;
    }
  }
  // console.log(decodes);
  return buildString(decodes, K);
};

function buildString(strings, k) {
  var result = "";
  var stack = [strings.shift()];
  while (stack.length > 0) {
    var s = stack.shift();
    var r = result + s.str;
    result = "";
    console.log({result, r, s});
    console.log(r.length)

    for (var i=0;i<s.repeats;i++) {
      result += r;
      if (result.length >= k) {
        return result[k-1];
      }
    }
    stack.push(strings.shift());
  }
}
