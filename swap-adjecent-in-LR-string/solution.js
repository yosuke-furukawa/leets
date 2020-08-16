/**
 * @param {string} start
 * @param {string} end
 * @return {boolean}
 */
var canTransform = function(start, end) {
  let R = 0
    let L = 0
    if (start.length != end.length) {
        return false
    }
    var i = 0
    while (i < start.length) {
        let char_s = start[i]
        let char_e = end[i] 
        if (char_s === 'R') {
            R += 1
        } else if (char_s === 'L') {
            L += 1
        }     
        if (L < 0 && R > 0) {
            return false
        }        
        
        if (char_e === 'R') {
            R -= 1
        } else if (char_e === 'L') {
            L -= 1
        }
        if (L > 0 || R < 0) {
            return false
        }
        if (L < 0 && R > 0) {
            return false
        }   
        i++
    }
    
    return L === 0 && R === 0
};
