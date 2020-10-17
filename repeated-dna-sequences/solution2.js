/**
 * @param {string} s
 * @return {string[]}
 */
var findRepeatedDnaSequences = function(s) {
  let sequenceMap = new Map();
  let result = [];
  if (s.length < 10) {
    return result;
  }
  for (let i = 0; i < s.length - 9; i++) {
    let substring = s.slice(i, i+10);
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
