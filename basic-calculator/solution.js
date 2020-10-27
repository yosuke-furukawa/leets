/**
 * @param {string} s
 * @return {number}
 */
var calculate = function(s) {
  s = s.replace(/\s/g, '');
    s += '+';
    const len = s.length;
    let index = 0;
    
    const calculateHelper = () => {
        let num = 0;
        let sum = 0;
        let prevSum = 0;
        let prevOp = '+';
        while (index < len) {
            const char = s[index];
            index += 1;
            
            if('0' <= char && char <= '9') {
                num = num * 10 + (char - '0');
            } else if (char === '(') {
                num = calculateHelper();
            } else {
                switch (prevOp) {
                    case '+':
                        sum += prevSum;
                        prevSum = num;
                        break;
                    case '-':
                        sum += prevSum;
                        prevSum = -num;
                        break;
                    default:
                        break;
                }
                
                if(char === ')') {
                    break;
                }
                
                prevOp = char;
                num = 0;
            }
        }
        return sum + prevSum;
    }
    
    return calculateHelper();
};
