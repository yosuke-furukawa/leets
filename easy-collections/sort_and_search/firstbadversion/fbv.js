/ * 
 * @param {integer} version number
 * @return {boolean} whether the version is bad
 * isBadVersion = function(version) {
 *     ...
 * };
 */

/**
 * @param {function} isBadVersion()
 * @return {function}
 */
var solution = function(isBadVersion) {
    /**
     * @param {integer} n Total versions
     * @return {integer} The first bad version
     */
    return function(n) {
        let left  = 0 
        let right = n
        let result = -1
        while(left <= right) {
            const middle = Math.floor(left + (right - left)/2)
            if(isBadVersion(middle)) {
                right = middle - 1
                result = middle
            } else {
                left = middle + 1
            }
        }
        return result
    };
};
