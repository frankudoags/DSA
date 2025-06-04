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

function removeElements(head: ListNode | null, val: number): ListNode | null {
    if(!head) return head;

    let curr = head;

    while(curr.next) {
        if(curr.next.val == val) curr.next = curr.next.next;
        else curr = curr.next;
    }
    
    if(head.val == val) head = head.next;
    return head;

};