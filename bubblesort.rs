
/*
 algo from wikipedia

procedure bubbleSort( A : list of sortable items )
    n = length(A)
    repeat 
        swapped = false
        for i = 1 to n-1 inclusive do
            /* if this pair is out of order */
            if A[i-1] > A[i] then
                /* swap them and remember something changed */
                swap( A[i-1], A[i] )
                swapped = true
            end if
        end for
    until not swapped
end procedure
*/

fn bubblesort<T: Ord>(els: &mut [T]) {
  let n = els.len();
  if n < 2 {
    return;
  }
  let mut swapped = true;
  while swapped {
    swapped = false;
    for i in 1..n {
      if els[i - 1] > els[i] {
        els.swap(i - 1, i);
        swapped = true;
      }
    }
  }
}

fn main() {
  let mut tosort = [3, 2, 1];
  println!("Hello Bubblesort {:?}", tosort);
  bubblesort(&mut tosort);
  println!("Sorted {:?}", tosort);
}