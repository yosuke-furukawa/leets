var partition = function(s) {
    function backtracking (position) {
        if (position >= len)
            partitions.push(Array.from(current));
        else {
            let currentSubstring = [];
            for (let index = position; index < len; index++) {
                currentSubstring.push(s[index]);
                if (s[position] === s[index] && isPalindrome(currentSubstring.join(''))) {
                    current.push(currentSubstring.join(''));
                    backtracking(index + 1);
                    current.pop();
                }
            }
        }
    };
	
    const len = s.length;
    const partitions = [];
    const current = [];
    backtracking(0);
    return partitions;
};

function isPalindrome(word) {
    let initIndex = 0;
    let endIndex = word.length - 1;
    while (initIndex <= endIndex) {
        if (word[initIndex] !== word[endIndex])
            return false;
        initIndex++;
        endIndex--;
    }
    return true;
}
