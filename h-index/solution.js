/**
 * @param {number[]} citations
 * @return {number}
 */
var hIndex = function(citations) {
  if (citations.length === 0) {
    return 0;
  }
  var cs = citations.sort((a,b) => b-a);
  for (var i=0;i<cs.length;i++) {
    var citation = cs[i];
    if (i - citation >= 0) {
      return i;
    }
  }
  return cs.filter(Boolean).length;
};
