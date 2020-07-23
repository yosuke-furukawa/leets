const searchWord = (start, s, wordDict, retArr, path) => {
    const n = s.length;
    if (start === n) {
        retArr.push(path.join(" "));
        return;
    }
    for (let i = start; i<n; i++) {
        const tempS = s.substring(start, i+1);
        const inDict = wordDict.includes(tempS);
        if (inDict) {
            searchWord(i+1, s, wordDict, retArr, [...path, tempS]);
        }
    }
}

var wordBreak1 = function(s, wordDict) {
    const n = s.length;
    dp = new Array(n+1);
    dp[0] = true;
    for (let i = 1; i < n+1; i++) {
        for (let j = i-1; j>=0; j--) {
            if (dp[j]) {
                const tempStr = s.substring(j, i);
                if (wordDict.includes(tempStr)) {
                    dp[i] = true;
                    break;
                }
            }
        }
    }
    return dp[n] !== undefined;
};

var wordBreak = function(s, wordDict) {
    let retArr = [];
    if (wordBreak1(s, wordDict) === false) return retArr;
    let path = [];
    searchWord(0, s, wordDict, retArr, path);
    return retArr;
};
