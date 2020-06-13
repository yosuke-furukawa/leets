/**
 * @param {string[]} strs
 * @return {string[][]}
 */
var groupAnagrams = function(strs) {
    var result = [];
    var memo = {};
    for (var i=0;i<strs.length;i++) {
        var str = strs[i];
        var sorted = [...str].sort().join();
        var arr = memo[sorted] || [];
        arr.push(str);
        if (arr.length === 1) {
            memo[sorted] = arr;
            result.push(arr);
        }
    }
    return result;
};
