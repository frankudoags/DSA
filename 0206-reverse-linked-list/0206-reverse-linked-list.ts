/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */
function reverseList(head: ListNode | null): ListNode | null {
    if(!head || !head.next) return head;
    let curr = head;
    let reverse = null;

    while(curr) {
        let next = curr.next;
        curr.next = reverse;
        reverse = curr;
        curr = next;
    }

    return reverse
};