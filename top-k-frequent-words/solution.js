/**
 * @param {string[]} words
 * @param {number} k
 * @return {string[]}
 */
var topKFrequent = function(words, k) {
   let map = words.sort().reduce((acc, curr) => {
      acc[curr] = (acc[curr] || 0 ) +1;
      return acc
    }, {})

    let sortedMap = Object.keys(map).sort((a, b) => map[b]-map[a]);


    return sortedMap.slice(0, k);
};
