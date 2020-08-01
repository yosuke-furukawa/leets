/**
 * @param {string} S
 * @param {number[]} indexes
 * @param {string[]} sources
 * @param {string[]} targets
 * @return {string}
 */
var findReplaceString = function(S, indexes, sources, targets) {
  var result = String(S);
  var replacement = 0;
  var sourceMaps = sources.map((source, i) => ({source, index: indexes[i]}));
  var targetMaps = targets.map((target, i) => ({target, index: indexes[i]}));
  sources = sourceMaps.sort((a, b) => a.index - b.index).map((sourceMap) => sourceMap.source);
  targets = targetMaps.sort((a, b) => a.index - b.index).map((targetMap) => targetMap.target);
  indexes = indexes.sort((a, b) => a-b);
  
  for (var i=0; i<indexes.length;i++) {
    var source = sources[i];
    var target = targets[i];
    var index = indexes[i];
    if (S.slice(index).indexOf(source) === 0) {
      var pre = result.slice(0, index+replacement);
      var t = result.slice(index+replacement, index+replacement+source.length);
      var post = result.slice(index+replacement+source.length);
      
      result = pre + target + post;
      replacement += target.length - source.length;
    }
  }
  return result;
};
