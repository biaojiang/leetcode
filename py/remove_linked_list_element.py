# （版本一）虚拟头节点法
# Definition for singly-linked list.
# The Optional type hint is typically used when a variable can either be of a certain type
# or None
from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def remove_elements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        # 创建虚拟头部节点以简化删除过程
        dummy_head = ListNode(next=head)

        # 遍历列表并删除值为val的节点
        current = dummy_head
        while current.next:
            if current.next.val == val:
                current.next = current.next.next
            else:
                current = current.next

        return dummy_head.next


def test_remove_elements():
    # Test case 1
    solution = Solution()
    head = ListNode(
        1, ListNode(2, ListNode(6, ListNode(3, ListNode(4, ListNode(5, ListNode(6))))))
    )
    result = solution.remove_elements(head, 6)
    expected_result = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
    assert compare_linked_lists(
        result, expected_result
    ), f"Test Case 1 Failed: {result}"

    # Add more test cases as needed


def compare_linked_lists(list1, list2):
    while list1 and list2:
        if list1.val != list2.val:
            return False
        list1 = list1.next
        list2 = list2.next
    return not list1 and not list2


if __name__ == "__main__":
    test_remove_elements()
    print("All tests passed.")
