/**
 * @param {number} numRows
 * @return {number[][]}
 */
var generate = function(numRows) {
    const result = [];
    for (var i=0; i<numRows; i++) {
        if (result[i-1]) {
            var prev = result[i-1];
            var next = [];
            for (var j=0;j<=prev.length;j++) {
                var p = prev[j-1] ?? 0;
                var n = prev[j] ?? 0;
                next.push(p + n);
            }
            result.push(next);
        } else {
          // first item [1]
          result.push([1]);    
        }
    }
    return result;
};
