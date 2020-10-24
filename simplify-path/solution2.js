/**
 * @param {string} path
 * @return {string}
 */
var simplifyPath = function(p) {
  var current = [];
  
  var ps = p.split("/").filter(Boolean).filter((a) => a != ".");
  for (const w of ps) {
    if (w === "..") {
      current.pop();
      continue;
    }
    current.push(w);
  }
  return "/" + current.join("/");
};
