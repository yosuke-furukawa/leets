function matchOne(s, p) {
  if (!p) return true;
  if (!s) return false;
  if (p === ".") return true;
  return p === s;
}

function matchStar(s, p) {
  return (matchOne(s[0], p[0]) && match(s.slice(1), p)) || match(s, p.slice(2));
}

function match(s, p) {
  if (p === "") {
    return s.length === 0;
  } else if (p[1] === "*") {
    return matchStar(s, p) 
  } else {
    return matchOne(s[0], p[0]) && match(s.slice(1), p.slice(1));
  }
};

/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
var isMatch = function(s, p) {
  return match(s, p);
}
