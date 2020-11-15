/**
 * @param {string} word1
 * @param {string} word2
 * @return {boolean}
 */
var closeStrings = function(word1, word2) {
  if (word1.length !== word2.length) {
    return false;
  }
  
  return isSame(word1, word2);
};

function isSame(word1, word2) {
  var word1Map = new Map();
  var word2Map = new Map();
  var word1Set = new Set([...word1]);
  var word2Set = new Set([...word2]);
  
  for (var c of word1Set) {
    if (!word2Set.has(c)) {
      return false;
    }
  }

  for (var w1 of word1) {
    if (word1Map.has(w1)) {
      word1Map.set(w1, word1Map.get(w1) + 1);
      continue;
    }
    word1Map.set(w1, 1);
  }
  
  for (var w2 of word2) {
    if (word2Map.has(w2)) {
      word2Map.set(w2, word2Map.get(w2) + 1);
      continue;
    }
    word2Map.set(w2, 1);
  }
  var maparray1 = [];
  for (var [key, value] of word1Map.entries()) {
    maparray1.push([key, value]);
  }
  
  var maparray2 = [];
  for (var [key, value] of word2Map.entries()) {
    maparray2.push([key, value]);
  }
  
  maparray1 = maparray1.sort((a, b) => a[1] - b[1]);
  maparray2 = maparray2.sort((a, b) => a[1] - b[1]);

  for (var i=0;i<maparray1.length;i++) {
    if (maparray2[i][1] !== maparray1[i][1]) {
      return false;
    }
  }
  return true;
}
