var isAnagram = function(s, t) {
  if (s.length !== t.length) {
    return false;
  }
  var a1 = s.split("").sort().join("");
  var a2 = t.split("").sort().join("");
  return a1 === a2;
};
