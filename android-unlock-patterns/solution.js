/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
let jumps = Array.from(new Array(10), () => new Array(10))

jumps[1][3] = jumps[3][1] = 2;
jumps[4][6] = jumps[6][4] = 5;
jumps[7][9] = jumps[9][7] = 8;
jumps[1][7] = jumps[7][1] = 4;
jumps[2][8] = jumps[8][2] = 5;
jumps[3][9] = jumps[9][3] = 6;
jumps[1][9] = jumps[9][1] = jumps[3][7] = jumps[7][3] = 5;

var numberOfPatterns = function(m, n) {
    let count = 0
    buildPattern()
    return count
    
    function buildPattern(curr = '', used = new Set) {
        if (curr.length >= m) {
            count++
        }
        if (curr.length === n) {
            return
        }
        for (let i = 1; i < 10; i++) {
            if (used.has(i)) continue
            let jumping = jumps[i][curr[curr.length - 1]]
            if (jumping) {
                if (!used.has(jumping)) continue
            }
            used.add(i)
            buildPattern(curr + i, used)
            used.delete(i)
        }
    }
};
