/**
 * @param {string} s
 * @return {string}
 */

var removeDuplicateLetters = function(s) {
  const stack = [];
  const seen = new Set();
  const lastSeen = new Map();
  for (var i=0;i<s.length;i++) {
    lastSeen.set(s[i], i);
  }
  
  for (var i=0;i<s.length;i++) {
    const c = s[i];
    
    if (!seen.has(c)) {
      while(stack.length > 0 && c.codePointAt() < stack[stack.length-1].codePointAt() && lastSeen.get(stack[stack.length-1]) > i){
        seen.delete(stack.pop());
      }
      seen.add(c);
      stack.push(c);
    }
  }
  
  return stack.join("");
};
