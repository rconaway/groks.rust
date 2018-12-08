extern crate array_init;

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
mod tests {

    #[test]
    fn initialize_arrays_by_specifying_each_element() {

        struct Record {
            a:isize,
            b:isize,
        }

        let a = [Record{a:1, b:10}, Record{a:2, b:20}];

        assert_eq!(a.len(), 2);

    }

    #[test]
    fn initialize_arrays_by_copying_single_value() {

        #[derive(Copy, Clone)]
        struct Record {
            a:isize,
            b:isize,
        }

        let a = [Record{a:1, b:10}; 5];

        assert_eq!(a.len(), 5);

    }

    #[test]
    fn initialize_as_separate_step() {
        use std::mem;

        #[derive(Copy, Clone)]
        struct Record {
            a:isize,
            b:isize,
        }

        let mut array:[Record;10] = unsafe {
            mem::uninitialized()
        };

            for i in 0..10 {
                array[i] = Record{a:i as isize, b:i as isize*10};
            }

        let x = &array[0];

        assert_eq!(array[1].a, 1);
        assert_eq!(array[1].b, 10);

    }

    #[test]
    fn initialize_with_algorithm() {
        use array_init;

        struct Record {
            a:isize,
            b:isize,
        }

        let a: [Record; 10] = array_init::array_init(|i| Record{a: (i * i) as isize, b: (i * i * i) as isize});

        assert_eq!(a.len(), 10);
        assert_eq!(a[2].a, 4);
        assert_eq!(a[2].b, 8);
    }


}