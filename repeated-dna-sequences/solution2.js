/**
 * @param {string} s
 * @return {string[]}
 */
var findRepeatedDnaSequences = function(s) {
  const sequenceMap = new Map();
  const result = [];
  if (s.length < 10) {
    return result;
  }
  for (let i = 0; i < s.length - 9; i++) {
    const substring = s.substring(i, i+10);
    if (sequenceMap.has(substring)) {
      if (sequenceMap.get(substring) < 2) {
        sequenceMap.set(substring, 2);
        result.push(substring);
      }
    } else {
      sequenceMap.set(substring, 1);
    }
  }
  return result;
};
