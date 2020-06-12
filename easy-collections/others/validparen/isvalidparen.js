/**
 * @param {string} s
 * @return {boolean}
 */
var parens = {
    "}" : "{",
    ")" : "(",
    "]" : "[",
};
var isValid = function(s) {
    var stacks = [];
    for (var i=0; i<s.length;i++) {
        if (parens[s[i]]) {
            if (stacks[0] !== parens[s[i]]) {
                return false;
            }
            stacks.shift();
        } else {
            stacks.unshift(s[i]);
        }
    }
    if (stacks.length === 0) {
        return true;
    } else {
        return false;
    }
};
