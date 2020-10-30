fn quicksort<T, F>(arr: &mut [T], left: isize, right: isize, compare: &F) 
    where F: fn(&T, &T) - Ordering {
        if right <= left { return }

        let mut i: isize = left - 1;
        let mut j: isize = right;
        let mut p: isize = i;
        let mut q: isize = j; 

        unsafe {
            
        }
    }
