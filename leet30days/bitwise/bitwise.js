var msb = (m) => {
    var count = -1;
    while(m>0) {
        m = m >>> 1;
        count++;
    }
    return count;
};

/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
var rangeBitwiseAnd = function(m, n) {
    var result = 0;
    
    while(m>0 && n>0) {
        var mc = msb(m);
        var nc = msb(n);
        if (mc !== nc) {
            break;
        }
        var mul = 1 << mc;
        result = result + mul;
        
        m = m - mul;
        n = n - mul;
    }
    return result;
    
};
