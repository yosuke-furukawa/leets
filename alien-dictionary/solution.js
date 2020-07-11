function topologicalSortHelper(node, visited, temp, graph, result, allwords) {
  temp[node] = true;
  allwords.delete(node);
  var neighbors = graph[node] || [];
  for (var i = 0; i < neighbors.length; i += 1) {
    var n = neighbors[i];
    if (temp[n]) {
      throw new Error("cyclic error");
    }
    if (!visited[n]) {
      topologicalSortHelper(n, visited, temp, graph, result, allwords);
    }
  }
  temp[node] = false;
  visited[node] = true;
  result.push(node);
}

/**
 * @param {string[]} words
 * @return {string}
 */
var alienOrder = function(words) {
  var diffs = [];
  var allwords = new Set();
  for (var i=0;i<words.length-1;i++) {
    var word1 = words[i];
    var word2 = words[i+1];
    for (var w1 of word1) {
      allwords.add(w1);
    }
    for (var w2 of word2) {
      allwords.add(w2);
    }
    var len = Math.min(word1.length, word2.length);
    var dic = [];
    for (var j=0;j<len;j++) {
      if (word1[j] !== word2[j]) {
        dic.push(word1[j],word2[j]);
        break;
      }
    }
    if (dic.length > 0) {
      diffs.push(dic);
    }
  }
  
  var dict = {};
  for (var diff of diffs) {
    if (dict[diff[0]] != null) {
      dict[diff[0]].push(diff[1]);
    } else {
      dict[diff[0]] = [diff[1]];
    }
  }
  
  var result = [];
  var visited = [];
  var temp = [];
  if (Object.keys(dict) == 0) {
    var maxIndex = 0;
    var max = 0;
    for (var i=0; i< words.length;i++) {
      if (words[i].length >= max) {
        maxIndex = i;
        max = words[i].length;
      } else {
        return "";
      }
    }
    return words[maxIndex];
  }
  for (var node in dict) {
    if (!visited[node] && !temp[node]) {
      try {
        topologicalSortHelper(node, visited, temp, dict, result, allwords);
      } catch(e) {
        return "";
      }
    }
  }
  var ans = result.reverse();
  if (allwords.size > 0) {
    for (var a of allwords.values()) {
      ans.push(a);
    }
  }
  return ans.join("");
};
