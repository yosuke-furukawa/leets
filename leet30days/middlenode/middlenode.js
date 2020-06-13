var middleNode = function(head) {
    let slow = head, fast = head;
    while (fast) {
        if (fast.next !== null && fast.next.next !== null) {
            slow = slow.next;
            fast = fast.next.next;
        }
        else if (fast.next) {
            slow = slow.next;
            break;
        }
        else {
            break;
        }
        
    }
    return slow;
};
