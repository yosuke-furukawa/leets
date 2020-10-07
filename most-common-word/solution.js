/**
 * @param {string} paragraph
 * @param {string[]} banned
 * @return {string}
 */
var mostCommonWord = function(paragraph, banned) {
  const p = paragraph.replace(/[!?',;.]/gi, " ").toLowerCase();
  const words = p.split(" ").map((w) => w.trim()).filter(Boolean);
  const bannedSet = new Set(banned);
  
  const map = new Map;
  for (const word of words) {
    if (bannedSet.has(word)) {
      continue;
    }
    if (map.has(word)) {
      map.set(word, map.get(word)+1);
      continue;
    }
    map.set(word, 1);
  }
  
  var max = 0;
  var ans = "";
  for (const [key, value] of map.entries()) {
    if (value > max) {
      max = value;
      ans = key;
    }
  }
  
  return ans;
};
