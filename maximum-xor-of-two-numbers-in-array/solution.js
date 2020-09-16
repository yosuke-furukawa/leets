/**
 * @param {number[]} nums
 * @return {number}
 */
var findMaximumXOR = function(nums) {
  const trie = makeTrie(nums)
  let max = 0
  for (const num of nums) {
    max = Math.max(max, search(trie, num))
  }  
  return max
};

function makeTrie (nums) {  
    const root = {}
    for (const num of nums) {
      let node = root
      for (let i = 30; i >= 0; i--) {
          const bit = (num >> i) & 1
          node[bit] = node[bit] || {}
          node = node[bit]
      }
    }
    return root
}

function search (trie, num) {
    let xor = 0
    let node = trie    
    for (let i = 30; i >= 0; i--) {
        const bit = (num >> i) & 1
        if (node[1-bit]) {
            xor += 2 ** i
            node = node[1-bit]
        } else {
            node = node[bit]
        }
    }
    return xor
}

