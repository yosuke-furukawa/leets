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
        var low  = 0 
        var high = n
        var mid = Math.floor((low + high)/2)
        var result = mid;
        while(low <= high) {
            mid = Math.floor((low + high)/2)
            if(isBadVersion(mid)) {
                high = mid - 1            
                result = mid;
            } else {
                low = mid + 1
            }
        }
        return result
    };
};
