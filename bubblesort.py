# algo from wikipedia
# procedure bubbleSort( A : list of sortable items )
#     n = length(A)
#     repeat 
#         swapped = false
#         for i = 1 to n-1 inclusive do
#             /* if this pair is out of order */
#             if A[i-1] > A[i] then
#                 /* swap them and remember something changed */
#                 swap( A[i-1], A[i] )
#                 swapped = true
#             end if
#         end for
#     until not swapped
# end procedure


def bubblesort(els):
  n = len(els)
  if n < 2:
    return
  swapped = True
  while swapped:
    swapped = False
    for i in range(1, n):
      if els[i - 1] > els[i]:
        els[i - 1], els[i] = els[i], els[i - 1]
        swapped = True

tosort = [3, 2, 1]

print("hello bubblesort", tosort)

bubblesort(tosort)

print("sorted", tosort)

