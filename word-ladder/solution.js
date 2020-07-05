/**
 * @param {string} beginWord
 * @param {string} endWord
 * @param {string[]} wordList
 * @return {number}
 */

var distance = function(word1, word2) {
  var len = word1.length;
  var dist = 0;
  for (var w1=0; w1<len;w1++) {
    if (word1[w1] !== word2[w1]) dist++;
  }
  return dist;
}

var ladderLength = function(beginWord, endWord, wordList) {
  var begin = beginWord;
  var end = endWord;
  var queue = [begin];
  var count = 1;
  while(queue.length > 0) {
    console.log(queue);
    var targets = [];
    while(queue.length > 0) {
      targets.push(queue.pop());
    }
    if (targets.includes(end)) {
      return count;
    }
    for (var target of targets) {
      for (var w=0;w<wordList.length;) {
        var word = wordList[w];
        if (distance(word, target) === 1) {
          wordList.splice(w, 1);
          queue.unshift(word);
          continue;
        }
        w++;
      }
    }
    count++;
  }
  return 0;
};
