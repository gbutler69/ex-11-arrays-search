mod rotate {
    use super::super::rotate;

    fn default_test_data(size: usize) -> Vec<u32> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .into_iter()
            .take(size)
            .collect()
    }

    #[test]
    fn it_works_for_a_zero_element_slice() {
        let mut input = default_test_data(0);
        rotate(&mut input, 1);
        assert_eq!(Vec::<u32>::new(), input);
    }

    #[test]
    fn it_works_for_a_single_element_slice() {
        let mut input = default_test_data(1);
        rotate(&mut input, 1);
        assert_eq!(vec![0], input);
    }

    #[test]
    fn it_works_for_a_two_element_slice_with_a_rotation_of_zero() {
        let mut input = default_test_data(2);
        rotate(&mut input, 0);
        assert_eq!(vec![0, 1], input);
    }

    #[test]
    fn it_works_for_a_two_element_slice_with_a_rotation_of_one() {
        let mut input = default_test_data(2);
        rotate(&mut input, 1);
        assert_eq!(vec![1, 0], input);
    }

    #[test]
    fn it_works_for_a_two_element_slice_with_a_rotation_of_two() {
        let mut input = default_test_data(2);
        rotate(&mut input, 2);
        assert_eq!(vec![0, 1], input);
    }

    #[test]
    fn it_works_for_a_two_element_slice_with_a_rotation_of_three() {
        let mut input = default_test_data(2);
        rotate(&mut input, 3);
        assert_eq!(vec![1, 0], input);
    }

    #[test]
    fn it_works_for_a_ten_element_slice_with_various_rotations() {
        let mut input = default_test_data(10);
        rotate(&mut input, 0);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 1);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 2);
        assert_eq!(vec![2, 3, 4, 5, 6, 7, 8, 9, 0, 1,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 3);
        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 4);
        assert_eq!(vec![4, 5, 6, 7, 8, 9, 0, 1, 2, 3,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 9);
        assert_eq!(vec![9, 0, 1, 2, 3, 4, 5, 6, 7, 8,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 10);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 15);
        assert_eq!(vec![5, 6, 7, 8, 9, 0, 1, 2, 3, 4,], input);

        let mut input = default_test_data(10);
        rotate(&mut input, 105);
        assert_eq!(vec![5, 6, 7, 8, 9, 0, 1, 2, 3, 4,], input);
    }
}

mod bubble_sort {
    use super::super::bubble_sort;

    fn test_data(size: usize) -> Vec<u32> {
        vec![
            3, 9, 1, 0, 5, 12, 97, 12, 11, 10, 7, 133, 87, 92, 101, 5, 2, 6, 200, 1,
        ]
        .into_iter()
        .take(size)
        .collect()
    }

    #[test]
    fn it_sorts_correctly_with_an_empty_input() {
        let mut input = test_data(0);
        bubble_sort(&mut input);
        assert_eq!(Vec::<u32>::new(), input);
    }

    #[test]
    fn it_sorts_correctly_with_a_single_item_as_input() {
        let mut input = test_data(1);
        bubble_sort(&mut input);
        assert_eq!(vec![3], input);
    }

    #[test]
    fn it_sorts_correctly_with_two_items_in_the_input() {
        let mut input = test_data(2);
        bubble_sort(&mut input);
        assert_eq!(vec![3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_three_items_in_the_input() {
        let mut input = test_data(3);
        bubble_sort(&mut input);
        assert_eq!(vec![1, 3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input() {
        let mut input = test_data(20);
        bubble_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input_if_already_sorted() {
        let mut input = test_data(20);
        bubble_sort(&mut input);
        bubble_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }
}

mod insertion_sort {
    use super::super::insertion_sort;

    fn test_data(size: usize) -> Vec<u32> {
        vec![
            3, 9, 1, 0, 5, 12, 97, 12, 11, 10, 7, 133, 87, 92, 101, 5, 2, 6, 200, 1,
        ]
        .into_iter()
        .take(size)
        .collect()
    }

    #[test]
    fn it_sorts_correctly_with_an_empty_input() {
        let mut input = test_data(0);
        insertion_sort(&mut input);
        assert_eq!(Vec::<u32>::new(), input);
    }

    #[test]
    fn it_sorts_correctly_with_a_single_item_as_input() {
        let mut input = test_data(1);
        insertion_sort(&mut input);
        assert_eq!(vec![3], input);
    }

    #[test]
    fn it_sorts_correctly_with_two_items_in_the_input() {
        let mut input = test_data(2);
        insertion_sort(&mut input);
        assert_eq!(vec![3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_three_items_in_the_input() {
        let mut input = test_data(3);
        insertion_sort(&mut input);
        assert_eq!(vec![1, 3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input() {
        let mut input = test_data(20);
        insertion_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input_if_already_sorted() {
        let mut input = test_data(20);
        insertion_sort(&mut input);
        insertion_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }
}

mod selection_sort {
    use super::super::selection_sort;

    fn test_data(size: usize) -> Vec<u32> {
        vec![
            3, 9, 1, 0, 5, 12, 97, 12, 11, 10, 7, 133, 87, 92, 101, 5, 2, 6, 200, 1,
        ]
        .into_iter()
        .take(size)
        .collect()
    }

    #[test]
    fn it_sorts_correctly_with_an_empty_input() {
        let mut input = test_data(0);
        selection_sort(&mut input);
        assert_eq!(Vec::<u32>::new(), input);
    }

    #[test]
    fn it_sorts_correctly_with_a_single_item_as_input() {
        let mut input = test_data(1);
        selection_sort(&mut input);
        assert_eq!(vec![3], input);
    }

    #[test]
    fn it_sorts_correctly_with_two_items_in_the_input() {
        let mut input = test_data(2);
        selection_sort(&mut input);
        assert_eq!(vec![3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_three_items_in_the_input() {
        let mut input = test_data(3);
        selection_sort(&mut input);
        assert_eq!(vec![1, 3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input() {
        let mut input = test_data(20);
        selection_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input_if_already_sorted() {
        let mut input = test_data(20);
        selection_sort(&mut input);
        selection_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }
}

mod counting_sort {
    use super::super::counting_sort;

    fn test_data(size: usize) -> Vec<usize> {
        vec![
            3, 9, 1, 0, 5, 12, 97, 12, 11, 10, 7, 133, 87, 92, 101, 5, 2, 6, 200, 1,
        ]
        .into_iter()
        .take(size)
        .collect()
    }

    #[test]
    fn it_sorts_correctly_with_an_empty_input() {
        let mut input = test_data(0);
        counting_sort(&mut input);
        assert_eq!(Vec::<usize>::new(), input);
    }

    #[test]
    fn it_sorts_correctly_with_a_single_item_as_input() {
        let mut input = test_data(1);
        counting_sort(&mut input);
        assert_eq!(vec![3], input);
    }

    #[test]
    fn it_sorts_correctly_with_two_items_in_the_input() {
        let mut input = test_data(2);
        counting_sort(&mut input);
        assert_eq!(vec![3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_three_items_in_the_input() {
        let mut input = test_data(3);
        counting_sort(&mut input);
        assert_eq!(vec![1, 3, 9], input);
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input() {
        let mut input = test_data(20);
        counting_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }

    #[test]
    fn it_sorts_correctly_with_twenty_items_in_the_input_if_already_sorted() {
        let mut input = test_data(20);
        counting_sort(&mut input);
        counting_sort(&mut input);
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 5, 6, 7, 9, 10, 11, 12, 12, 87, 92, 97, 101, 133, 200,],
            input
        );
    }
}

mod chopsticks {
    use super::super::chopsticks;

    #[test]
    fn it_works_correctly_producing_a_total_number_of_2_pairs_with_a_delta_of_2_on_the_given_input()
    {
        assert_eq!(2, chopsticks(&mut [1, 3, 3, 9, 4], 2))
    }

    #[test]
    fn it_works_correctly_producing_a_total_number_of_3_pairs_with_a_delta_of_2_on_the_given_input()
    {
        assert_eq!(3, chopsticks(&mut [1, 3, 3, 9, 4, 5, 7], 2))
    }
}

mod kingdom_defense {
    use super::super::kingdom_defense::kingdom_defense;

    #[test]
    fn it_works() {
        assert_eq!(12, kingdom_defense(15, 8, &mut [(3, 8), (11, 2), (8, 6)]));
    }
}

mod shortest_path {
    use super::super::shortest_path::shortest_path;

    #[test]
    fn it_works() {
        assert_eq!("NNE", shortest_path("SNNNEWE"));
    }
}

mod palindrome {
    use super::super::palindrome::palindrome;

    #[test]
    fn it_works() {
        assert!(palindrome("radar"));
        assert!(!palindrome("radars"));
    }
}

mod permutation {
    use super::super::permutation::permutation;

    #[test]
    fn it_works() {
        assert!(permutation("abcdef", "edcfba"));
        assert!(!permutation("abcdefs", "dfcbaet"));
    }
}

mod remove_duplicates {

    use super::super::remove_duplicates::remove_duplicates;

    #[test]
    fn it_works() {
        assert_eq!("efgkors", remove_duplicates("geeksforgeeks"));
        assert_ne!("efgkors", remove_duplicates("geeksareforgeeks"));
    }
}

mod vowel_find {
    use super::super::vowel_find::vowel_find;

    #[test]
    fn it_works() {
        assert_eq!("aeoiiiaeaeiou", vowel_find("aeoithisisatestaeiouy"));
    }
}

mod binary_string {
    use super::super::binary_string::binary_string;

    #[test]
    fn it_works() {
        assert_eq!(0, binary_string("000"));
        assert_eq!(1, binary_string("0000001"));
        assert_eq!(2, binary_string("00000010"));
        assert_eq!(4, binary_string("00000100"));
        assert_eq!(5, binary_string("00000101"));
        assert_eq!(
            u32::MAX - 1,
            binary_string("11111111111111111111111111111110")
        );
        assert_eq!(u32::MAX, binary_string("11111111111111111111111111111111"));
    }
}

mod print_2d {
    use super::super::print_2d::*;

    #[test]
    fn spiral_print_works() {
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10],
            spiral_print(Vec2d::from(vec![
                vec![1, 2, 3, 4],     //
                vec![5, 6, 7, 8],     //
                vec![9, 10, 11, 12],  //
                vec![13, 14, 15, 16], //
            ]))
        )
    }

    #[test]
    fn reverse_wave_print_works() {
        assert_eq!(
            vec![4, 8, 12, 16, 15, 11, 7, 3, 2, 6, 10, 14, 13, 9, 5, 1],
            reverse_wave_print(Vec2d::from(vec![
                vec![1, 2, 3, 4],     //
                vec![5, 6, 7, 8],     //
                vec![9, 10, 11, 12],  //
                vec![13, 14, 15, 16], //
            ]))
        )
    }
}

mod search_2d {
    use super::super::print_2d::Vec2d;
    use super::super::search_2d::staircase_search;

    #[test]
    fn it_works() {
        assert_eq!(
            Some((1, 2)),
            staircase_search(
                Vec2d::from(vec![
                    vec![1, 2, 3, 4],     //
                    vec![5, 6, 7, 8],     //
                    vec![9, 10, 11, 12],  //
                    vec![13, 14, 15, 16], //
                ]),
                10
            )
        );
        assert_eq!(
            Some((0, 0)),
            staircase_search(
                Vec2d::from(vec![
                    vec![1, 2, 3, 4],     //
                    vec![5, 6, 7, 8],     //
                    vec![9, 10, 11, 12],  //
                    vec![13, 14, 15, 16], //
                ]),
                1
            )
        );
        assert_eq!(
            Some((3, 3)),
            staircase_search(
                Vec2d::from(vec![
                    vec![1, 2, 3, 4],     //
                    vec![5, 6, 7, 8],     //
                    vec![9, 10, 11, 12],  //
                    vec![13, 14, 15, 16], //
                ]),
                16
            )
        );
        assert_eq!(
            None,
            staircase_search(
                Vec2d::from(vec![
                    vec![1, 2, 3, 4],     //
                    vec![5, 6, 7, 8],     //
                    vec![9, 10, 11, 12],  //
                    vec![13, 14, 15, 16], //
                ]),
                0
            )
        );
        assert_eq!(
            None,
            staircase_search(
                Vec2d::from(vec![
                    vec![1, 2, 3, 4],     //
                    vec![5, 6, 7, 8],     //
                    vec![9, 10, 11, 12],  //
                    vec![13, 14, 15, 16], //
                ]),
                17
            )
        );
    }
}

mod mango {
    mod best_split {
        use super::super::super::mango::best_split;
        use super::super::super::print_2d::Vec2d;

        #[test]
        fn it_works() {
            assert_eq!(
                3,
                best_split(Vec2d::from(vec![
                    vec![0, 1, 1, 0, 0, 0], //
                    vec![1, 0, 0, 1, 1, 0], //
                    vec![0, 1, 0, 0, 0, 0], //
                    vec![0, 1, 1, 0, 0, 1], //
                    vec![1, 0, 0, 1, 1, 0], //
                    vec![0, 1, 0, 0, 0, 0]
                ]))
            )
        }
    }
}

mod pascal {
    mod pascals_triangle {
        use super::super::super::pascal::pascals_triangle;

        #[test]
        fn it_works_for_0() {
            assert_eq!(Vec::<Vec<u64>>::new(), pascals_triangle(0))
        }

        #[test]
        fn it_works_for_1() {
            assert_eq!(vec![vec![1]], pascals_triangle(1))
        }

        #[test]
        fn it_works_for_2() {
            assert_eq!(
                vec![
                    vec![1],    //
                    vec![1, 1]  //
                ],
                pascals_triangle(2)
            )
        }

        #[test]
        fn it_works_for_6() {
            assert_eq!(
                vec![
                    vec![1],                  //
                    vec![1, 1],               //
                    vec![1, 2, 1],            //
                    vec![1, 3, 3, 1],         //
                    vec![1, 4, 6, 4, 1],      //
                    vec![1, 5, 10, 10, 5, 1], //
                ],
                pascals_triangle(6)
            )
        }
    }
}

mod matrix {
    mod sub_matrix {
        mod sum {
            use super::super::super::super::matrix::*;

            #[test]
            fn it_works() {
                let matrix = QueryableMatrix2d::from((
                    5,
                    vec![
                        1, 2, 3, 4, 6, //
                        5, 3, 8, 1, 2, //
                        4, 6, 7, 5, 5, //
                        2, 4, 8, 9, 4,
                    ],
                    || 0,
                    |u: &mut u64, t: u32| *u += t as u64,
                    |u1: &mut u64, u2| *u1 += u2,
                    |u1: &mut u64, u2| *u1 -= u2,
                ));

                assert_eq!(1, matrix.reduced_submatrix_value(0, 0, 0, 0));
                assert_eq!(11, matrix.reduced_submatrix_value(0, 0, 1, 1));
                assert_eq!(38, matrix.reduced_submatrix_value(2, 2, 4, 3));
                assert_eq!(38, matrix.reduced_submatrix_value(2, 1, 3, 3));
            }
        }
    }
}

mod vector {

    use std::cell::Cell;

    use super::super::vector::Vector;

    #[test]
    fn it_allocates_a_zero_length_zero_capacity_vector_using_new() {
        let v = Vector::<u32>::new();
        assert_eq!(
            0,
            v.len(),
            "Vector created using 'Vector::new()' should be length of zero"
        );
        assert_eq!(
            0,
            v.capacity(),
            "Vector created using 'Vector::new()' should be capacity of zero"
        );
        assert!(
            v.is_empty(),
            "Vector created using 'Vector::new()' should be empty"
        );
    }

    #[test]
    fn it_allocates_a_zero_length_zero_capacity_vector_using_with_capacity_0() {
        let v = Vector::<u32>::with_capacity(0);
        assert_eq!(
            0,
            v.len(),
            "Vector created using 'Vector::with_capacity(0)' should be length of zero"
        );
        assert_eq!(
            0,
            v.capacity(),
            "Vector created using 'Vector::with_capacity(0)' should have capacity of zero"
        );
        assert!(
            v.is_empty(),
            "Vector created using 'Vector::with_capacity(0)' should be empty"
        );
    }

    #[test]
    fn it_allocates_a_zero_length_100_capacity_vector_using_with_capacity_100() {
        let v = Vector::<u32>::with_capacity(100);
        assert_eq!(
            0,
            v.len(),
            "Vector created using 'Vector::with_capacity(100)' should be length of zero"
        );
        assert_eq!(
            100,
            v.capacity(),
            "Vector created using 'Vector::with_capacity(100)' should have the requested capacity"
        );
        assert!(
            v.is_empty(),
            "Vector created using 'Vector::with_capacity(100)' should be empty"
        );
    }

    #[test]
    fn it_allows_pushing_and_then_popping_a_value_correctly_and_subsequent_and_prior_pops_return_none(
    ) {
        let mut v = Vector::<u32>::new();
        assert_eq!(None, v.pop());
        v.push(100);
        assert_eq!(100, v.pop().unwrap());
        assert_eq!(None, v.pop());
        assert_eq!(None, v.pop());
        v.push(200);
        assert_eq!(200, v.pop().unwrap());
    }

    #[test]
    fn it_drops_remaining_items_when_it_is_dropped() {
        struct MockDroppable<'a> {
            drop_callback: Box<dyn FnMut() + 'a>,
        }
        impl<'a> MockDroppable<'a> {
            fn new<F>(drop_callback: F) -> Self
            where
                F: FnMut() + 'a,
            {
                Self {
                    drop_callback: Box::new(drop_callback),
                }
            }
        }
        impl<'a> Drop for MockDroppable<'a> {
            fn drop(&mut self) {
                (self.drop_callback)();
            }
        }

        let item_1_dropped = Cell::new(false);
        let item_2_dropped = Cell::new(false);
        let item_3_dropped = Cell::new(false);

        let mut v = Vector::<MockDroppable>::new();
        v.push(MockDroppable::new(|| item_1_dropped.set(true)));
        v.push(MockDroppable::new(|| item_2_dropped.set(true)));
        v.push(MockDroppable::new(|| item_3_dropped.set(true)));
        let item_3_popped = v.pop();
        drop(v);

        assert!(item_1_dropped.get());
        assert!(item_2_dropped.get());
        assert!(!item_3_dropped.get());
        assert!(item_3_popped.is_some());

        drop(item_3_popped);
        assert!(item_3_dropped.get());
    }

    #[test]
    fn it_allows_shared_immutable_access_to_items_via_indexing() {
        let mut v = Vector::<u32>::new();
        v.push(3);
        v.push(6);
        v.push(9);
        v.push(12);

        assert_eq!(3, v[0]);
        assert_eq!(6, v[1]);
        assert_eq!(9, v[2]);
        assert_eq!(12, v[3]);
    }

    #[test]
    fn it_allows_exclusive_mutable_access_to_items_via_indexing() {
        let mut v = Vector::<u32>::new();
        v.push(3);
        v.push(6);
        v.push(9);
        v.push(12);
        v[2] = 15;

        assert_eq!(3, v[0]);
        assert_eq!(6, v[1]);
        assert_eq!(15, v[2]);
        assert_eq!(12, v[3]);
    }

    #[test]
    #[should_panic(expected = "Index out-of-range: Length 4, Index 4")]
    fn it_panics_when_indexing_out_of_bounds_for_shared_nonmutable_access() {
        let mut v = Vector::<u32>::new();
        v.push(3);
        v.push(6);
        v.push(9);
        v.push(12);

        let _value = v[4];
    }

    #[test]
    #[should_panic(expected = "Index out-of-range: Length 4, Index 4")]
    fn it_panics_when_indexing_out_of_bounds_for_exclusive_mutable_access() {
        let mut v = Vector::<u32>::new();
        v.push(3);
        v.push(6);
        v.push(9);
        v.push(12);

        v[4] = 15;
    }
}

mod cabs {
    use super::super::cabs::sort_by_distance_from_origin;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                (-1, 1),  //
                (-3, -3), //
                (5, 5),   //
                (7, 7)    //
            ],
            sort_by_distance_from_origin(vec![
                (5, 5),   //
                (7, 7),   //
                (-3, -3), //
                (-1, 1),  //
            ])
        )
    }
}

mod fruit {
    use super::super::fruits::{sort_by, FruitSortKey};

    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                ("Apple".into(), 5),   //
                ("Banana".into(), 3),  //
                ("Cherry".into(), 10), //
                ("Donut".into(), 7),   //
                ("Eclare".into(), 9),  //
            ],
            sort_by(
                vec![
                    ("Cherry".into(), 10), //
                    ("Donut".into(), 7),   //
                    ("Banana".into(), 3),  //
                    ("Apple".into(), 5),   //
                    ("Eclare".into(), 9),  //
                ],
                FruitSortKey::Name
            )
        );
        assert_eq!(
            vec![
                ("Banana".into(), 3),  //
                ("Apple".into(), 5),   //
                ("Donut".into(), 7),   //
                ("Eclare".into(), 9),  //
                ("Cherry".into(), 10), //
            ],
            sort_by(
                vec![
                    ("Cherry".into(), 10), //
                    ("Donut".into(), 7),   //
                    ("Banana".into(), 3),  //
                    ("Apple".into(), 5),   //
                    ("Eclare".into(), 9),  //
                ],
                FruitSortKey::Cost
            )
        );
    }
}

mod image {
    use super::super::image::rotate_90_degrees;
    use super::super::print_2d::Vec2d;

    #[test]
    fn it_rotates_an_image_with_no_pixels_correctly() {
        let mut image = Vec2d::from(Vec::<Vec<u32>>::new());
        rotate_90_degrees(&mut image);
        assert_eq!(Vec2d::from(Vec::<Vec<u32>>::new()), image);
    }

    #[test]
    fn it_rotates_an_image_with_a_single_pixel_correctly() {
        let mut image = Vec2d::from(vec![vec![10]]);
        rotate_90_degrees(&mut image);
        assert_eq!(Vec2d::from(vec![vec![10]]), image);
    }

    #[test]
    fn it_rotates_a_3_by_3_pixel_image_correctly() {
        let mut image = Vec2d::from(vec![
            vec![1, 2, 3], //
            vec![4, 5, 6], //
            vec![7, 8, 9], //
        ]);
        rotate_90_degrees(&mut image);
        assert_eq!(
            Vec2d::from(vec![
                vec![7, 4, 1], //
                vec![8, 5, 2], //
                vec![9, 6, 3], //
            ]),
            image
        );
    }

    #[test]
    fn it_rotates_a_5_by_5_pixel_image_correctly() {
        let mut image = Vec2d::from(vec![
            vec![1, 2, 3, 4, 5],      //
            vec![6, 7, 8, 9, 10],     //
            vec![11, 12, 13, 14, 15], //
            vec![16, 17, 18, 19, 20], //
            vec![21, 22, 23, 24, 25], //
        ]);
        rotate_90_degrees(&mut image);
        assert_eq!(
            Vec2d::from(vec![
                vec![21, 16, 11, 6, 1],  //
                vec![22, 17, 12, 7, 2],  //
                vec![23, 18, 13, 8, 3],  //
                vec![24, 19, 14, 9, 4],  //
                vec![25, 20, 15, 10, 5], //
            ]),
            image
        );
    }

    #[test]
    fn it_rotates_a_2_by_2_pixel_image_correctly() {
        let mut image = Vec2d::from(vec![
            vec![1, 2], //
            vec![3, 4], //
        ]);
        rotate_90_degrees(&mut image);
        assert_eq!(
            Vec2d::from(vec![
                vec![3, 1], //
                vec![4, 2], //
            ]),
            image
        );
    }

    #[test]
    fn it_rotates_a_4_by_4_pixel_image_correctly() {
        let mut image = Vec2d::from(vec![
            vec![1, 2, 3, 4],     //
            vec![5, 6, 7, 8],     //
            vec![9, 10, 11, 12],  //
            vec![13, 14, 15, 16], //
        ]);
        rotate_90_degrees(&mut image);
        assert_eq!(
            Vec2d::from(vec![
                vec![13, 9, 5, 1],  //
                vec![14, 10, 6, 2], //
                vec![15, 11, 7, 3], //
                vec![16, 12, 8, 4], //
            ]),
            image
        );
    }

    #[test]
    #[should_panic(expected = "image must be square")]
    fn it_panics_on_an_image_where_the_width_does_not_equal_the_height() {
        let mut image = Vec2d::from(vec![
            vec![1, 2, 3, 4],     //
            vec![5, 6, 7, 8],     //
            vec![9, 10, 11, 12],  //
            vec![13, 14, 15, 16], //
            vec![17, 18, 19, 20], //
        ]);
        // this will panic
        rotate_90_degrees(&mut image);
    }
}

mod zeroes {
    use super::super::print_2d::Vec2d;
    use super::super::zeroes::make_zeroes;

    #[test]
    fn it_works() {
        let input = Vec2d::<u32>::from(vec![
            vec![1, 2, 3, 4, 5],      //
            vec![6, 0, 8, 9, 10],     //
            vec![11, 12, 13, 14, 15], //
            vec![16, 17, 0, 19, 20],  //
            vec![21, 22, 23, 24, 0],  //
        ]);
        let expected = Vec2d::<u32>::from(vec![
            vec![1, 0, 0, 4, 0],   //
            vec![0, 0, 0, 0, 0],   //
            vec![11, 0, 0, 14, 0], //
            vec![0, 0, 0, 0, 0],   //
            vec![0, 0, 0, 0, 0],   //
        ]);
        let actual = make_zeroes(input);
        assert_eq!(expected, actual);
    }
}

mod xoring {
    use super::super::xoring::find_unique_number;

    #[test]
    fn it_returns_none_when_the_input_is_empty_by_returning_none() {
        let input = vec![];
        let actual = find_unique_number(input);
        assert_eq!(None, actual);
    }

    #[test]
    fn it_works_when_there_is_a_single_number_by_returing_that_number() {
        let input = vec![7];
        let expected = 7;
        let actual = find_unique_number(input);
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn it_works_when_there_is_no_unique_number_by_returning_none() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected = None;
        let actual = find_unique_number(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_works_when_there_is_a_single_unique_number_by_returning_that_number() {
        let input = vec![1, 2, 3, 4, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected = 5;
        let actual = find_unique_number(input);
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn it_works_when_there_is_a_single_unique_number_which_is_zero_by_returning_zero() {
        let input = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let expected = 0;
        let actual = find_unique_number(input);
        assert_eq!(Some(expected), actual);
    }
}

mod modulus {
    use super::super::modulus::powmod;

    #[test]
    fn it_works() {
        assert_eq!(9603, powmod(12, 25, 10007));
    }
}

mod earth {
    use super::super::earth::level_up_by_powers_of_2;

    #[test]
    fn it_works_for_0_which_requires_0_jumps() {
        assert_eq!(0, level_up_by_powers_of_2(0));
    }

    #[test]
    fn it_works_for_1_which_requires_1_jump() {
        assert_eq!(1, level_up_by_powers_of_2(1));
    }

    #[test]
    fn it_works_for_7_which_requires_3_jumps() {
        assert_eq!(3, level_up_by_powers_of_2(7));
    }

    #[test]
    fn it_works_for_39_which_requires_4_jumps() {
        assert_eq!(4, level_up_by_powers_of_2(39));
    }

    #[test]
    fn it_works_for_257_which_requires_2_jumps() {
        assert_eq!(2, level_up_by_powers_of_2(257));
    }
}

mod subset {
    use super::super::subset::subset_sum_queries;

    #[test]
    fn it_works() {
        let input_data = vec![1, 2, 3];
        let input_queries = vec![5, 3, 8];
        let expected = vec![true, true, false];

        let actual = subset_sum_queries(input_data, input_queries);

        assert_eq!(expected, actual);
    }
}

mod recursion {
    mod factorial {
        use super::super::super::recursion::factorial;

        #[test]
        fn it_works_for_5_producing_120() {
            assert_eq!(120, factorial(5));
        }

        #[test]
        fn it_works_for_10_producing_3_628_800() {
            assert_eq!(3_628_800, factorial(10));
        }

        #[test]
        fn it_works_for_20_producing_2_432_902_008_176_640_000() {
            assert_eq!(2_432_902_008_176_640_000, factorial(20));
        }

        #[test]
        #[should_panic(expected = "attempt to multiply with overflow")]
        fn it_panics_for_21_with_an_overflow() {
            factorial(21);
        }
    }

    mod fibonacci {
        use super::super::super::recursion::{fibonacci, fibonacci_tc};

        #[test]
        fn fibonacci_without_tail_call_optimization_works() {
            assert_eq!(0, fibonacci(0));
            assert_eq!(1, fibonacci(1));
            assert_eq!(1, fibonacci(2));
            assert_eq!(2, fibonacci(3));
            assert_eq!(3, fibonacci(4));
            assert_eq!(5, fibonacci(5));
            assert_eq!(8, fibonacci(6));
            assert_eq!(13, fibonacci(7));
            assert_eq!(21, fibonacci(8));
            assert_eq!(34, fibonacci(9));
            assert_eq!(55, fibonacci(10));
        }

        #[test]
        fn fibonacci_with_tail_call_optimization_works() {
            assert_eq!(0, fibonacci_tc(0));
            assert_eq!(1, fibonacci_tc(1));
            assert_eq!(1, fibonacci_tc(2));
            assert_eq!(2, fibonacci_tc(3));
            assert_eq!(3, fibonacci_tc(4));
            assert_eq!(5, fibonacci_tc(5));
            assert_eq!(8, fibonacci_tc(6));
            assert_eq!(13, fibonacci_tc(7));
            assert_eq!(21, fibonacci_tc(8));
            assert_eq!(34, fibonacci_tc(9));
            assert_eq!(55, fibonacci_tc(10));
        }
    }

    mod is_array_sorted {
        use super::super::super::recursion::{is_array_sorted, is_array_sorted_tc};

        #[test]
        fn is_array_sorted_without_tail_call_optimization_works() {
            assert!(is_array_sorted(&[0_u32; 0])); // empty array/length of zero
            assert!(is_array_sorted(&[3]));
            assert!(is_array_sorted(&[1, 2, 3, 4, 5]));
            assert!(!is_array_sorted(&[1, 3, 2, 4, 5]));
            assert!(!is_array_sorted(&[1, 2, 3, 7, 4, 5]));
        }

        #[test]
        fn is_array_sorted_tc_with_tail_call_optimization_works() {
            assert!(is_array_sorted_tc(&[0_u32; 0])); // empty array/length of zero
            assert!(is_array_sorted_tc(&[3]));
            assert!(is_array_sorted_tc(&[1, 2, 3, 4, 5]));
            assert!(!is_array_sorted_tc(&[1, 3, 2, 4, 5]));
            assert!(!is_array_sorted_tc(&[1, 2, 3, 7, 4, 5]));
        }
    }

    mod find_all_occurences {
        use super::super::super::recursion::find_all_occurences;

        #[test]
        fn it_works() {
            assert_eq!(
                vec![3, 6, 9],
                find_all_occurences(&3, &[1, 2, 5, 3, 1, 2, 3, 8, 6, 3, 6, 7, 9])
            );
        }

        #[test]
        fn it_works_when_the_number_is_not_found() {
            assert_eq!(
                Vec::<usize>::new(),
                find_all_occurences(&10, &[1, 2, 5, 3, 1, 2, 3, 8, 6, 3, 6, 7, 9])
            );
        }

        #[test]
        fn it_works_when_the_set_to_search_is_empty() {
            assert_eq!(Vec::<usize>::new(), find_all_occurences(&10, &[0_u32; 0]));
        }
    }

    mod tiling {
        use super::super::super::recursion::find_number_of_possible_tilings;

        #[test]
        fn it_works_when_the_tile_size_is_larger_than_the_floor_size_by_returning_zero() {
            assert_eq!(0, find_number_of_possible_tilings((5, 5), (5, 6)));
        }

        #[test]
        fn it_works_when_the_floor_size_and_tile_size_are_both_zero_by_returning_one() {
            assert_eq!(1, find_number_of_possible_tilings((0, 0), (0, 0)));
            assert_eq!(1, find_number_of_possible_tilings((0, 0), (0, 1)));
            assert_eq!(1, find_number_of_possible_tilings((0, 0), (1, 0)));
            assert_eq!(1, find_number_of_possible_tilings((0, 1), (0, 0)));
            assert_eq!(1, find_number_of_possible_tilings((0, 1), (0, 1)));
            assert_eq!(1, find_number_of_possible_tilings((0, 1), (1, 0)));
            assert_eq!(1, find_number_of_possible_tilings((1, 0), (0, 0)));
            assert_eq!(1, find_number_of_possible_tilings((1, 0), (0, 1)));
            assert_eq!(1, find_number_of_possible_tilings((1, 0), (1, 0)));
        }

        #[test]
        fn it_works_when_the_floor_size_is_zero_and_the_tile_size_is_anything_by_returning_one() {
            assert_eq!(1, find_number_of_possible_tilings((0, 0), (1, 1)));
            assert_eq!(1, find_number_of_possible_tilings((0, 1), (1, 1)));
            assert_eq!(1, find_number_of_possible_tilings((1, 0), (1, 1)));
        }

        #[test]
        fn it_works_when_the_floor_size_is_anything_and_the_tile_size_is_zero_by_returning_zero() {
            assert_eq!(0, find_number_of_possible_tilings((100, 100), (0, 0)));
            assert_eq!(0, find_number_of_possible_tilings((100, 100), (0, 1)));
            assert_eq!(0, find_number_of_possible_tilings((100, 100), (1, 0)));
        }

        #[test]
        fn it_works_when_the_tile_and_floor_size_match_by_returning_one() {
            assert_eq!(1, find_number_of_possible_tilings((1, 1), (1, 1)));
            assert_eq!(1, find_number_of_possible_tilings((10, 20), (10, 20)));
            assert_eq!(1, find_number_of_possible_tilings((20, 10), (20, 10)));
            assert_eq!(1, find_number_of_possible_tilings((10, 20), (20, 10)));
            assert_eq!(1, find_number_of_possible_tilings((20, 10), (10, 20)));
        }

        #[test]
        fn it_works_with_a_nonsquare_floor_and_nonsquare_tile() {
            assert_eq!(3, find_number_of_possible_tilings((4, 3), (1, 3)));
            assert_eq!(3, find_number_of_possible_tilings((4, 3), (3, 1)));
        }

        #[test]
        fn it_works_with_a_square_floor_and_a_square_tile_where_the_tile_fits_evenly() {
            assert_eq!(1, find_number_of_possible_tilings((20, 20), (5, 5)));
        }

        #[test]
        fn it_works_with_a_nonsquare_floor_and_a_square_tile_where_the_tile_fits_evenly() {
            assert_eq!(1, find_number_of_possible_tilings((20, 10), (5, 5)));
        }

        #[test]
        fn it_works_with_a_square_floor_and_a_nonsquare_tile_where_the_tile_fits_evenly() {
            assert_eq!(30, find_number_of_possible_tilings((4, 4), (1, 2)));
            assert_eq!(30, find_number_of_possible_tilings((4, 4), (2, 1)));
            assert_eq!(2, find_number_of_possible_tilings((4, 4), (2, 4)));
            assert_eq!(2, find_number_of_possible_tilings((4, 4), (4, 2)));
        }

        #[test]
        fn it_works_with_a_nonsquare_floor_and_a_nonsquare_tile_where_the_tile_fits_evenly() {
            assert_eq!(5, find_number_of_possible_tilings((2, 4), (1, 2)));
            assert_eq!(5, find_number_of_possible_tilings((2, 4), (2, 1)));
            assert_eq!(5, find_number_of_possible_tilings((4, 2), (1, 2)));
            assert_eq!(5, find_number_of_possible_tilings((4, 2), (2, 1)));
        }

        #[test]
        fn it_works_with_a_square_floor_and_a_nonsquare_tile_where_the_tile_does_not_fit_evenly() {
            assert_eq!(0, find_number_of_possible_tilings((4, 4), (1, 3)));
            assert_eq!(0, find_number_of_possible_tilings((4, 4), (3, 1)));
        }

        #[test]
        fn it_works_with_a_nonsquare_floor_and_a_square_tile_where_the_tile_does_not_fit_evenly() {
            assert_eq!(0, find_number_of_possible_tilings((4, 4), (1, 3)));
            assert_eq!(0, find_number_of_possible_tilings((4, 4), (3, 1)));
        }

        #[test]
        fn it_works_with_a_nonsquare_floor_and_a_nonsquare_tile_where_the_tile_does_not_fit_evenly()
        {
            assert_eq!(0, find_number_of_possible_tilings((7, 4), (1, 3)));
            assert_eq!(0, find_number_of_possible_tilings((7, 4), (3, 1)));
            assert_eq!(0, find_number_of_possible_tilings((4, 7), (1, 3)));
            assert_eq!(0, find_number_of_possible_tilings((4, 7), (3, 1)));
        }
    }

    mod binary_strings {
        use super::super::super::recursion::binary_strings_with_nonrepeating_ones_of_length_sorted;

        #[test]
        fn it_works_with_length_0() {
            assert_eq!(
                Vec::<String>::new(),
                binary_strings_with_nonrepeating_ones_of_length_sorted(0)
            )
        }

        #[test]
        fn it_works_with_length_1() {
            assert_eq!(
                vec![
                    "0", //
                    "1", //
                ],
                binary_strings_with_nonrepeating_ones_of_length_sorted(1)
            )
        }

        #[test]
        fn it_works_with_length_2() {
            assert_eq!(
                vec![
                    "00", //
                    "01", //
                    "10"
                ],
                binary_strings_with_nonrepeating_ones_of_length_sorted(2)
            )
        }

        #[test]
        fn it_works_with_length_3() {
            assert_eq!(
                vec![
                    "000", //
                    "001", //
                    "010", //
                    "100", //
                    "101"  //
                ],
                binary_strings_with_nonrepeating_ones_of_length_sorted(3)
            )
        }

        #[test]
        fn it_works_with_length_4() {
            assert_eq!(
                vec![
                    "0000", //
                    "0001", //
                    "0010", //
                    "0100", //
                    "0101", //
                    "1000", //
                    "1001", //
                    "1010", //
                ],
                binary_strings_with_nonrepeating_ones_of_length_sorted(4)
            )
        }

        #[test]
        fn it_works_with_length_12() {
            assert_eq!(
                vec![
                    "000000000000",
                    "000000000001",
                    "000000000010",
                    "000000000100",
                    "000000000101",
                    "000000001000",
                    "000000001001",
                    "000000001010",
                    "000000010000",
                    "000000010001",
                    "000000010010",
                    "000000010100",
                    "000000010101",
                    "000000100000",
                    "000000100001",
                    "000000100010",
                    "000000100100",
                    "000000100101",
                    "000000101000",
                    "000000101001",
                    "000000101010",
                    "000001000000",
                    "000001000001",
                    "000001000010",
                    "000001000100",
                    "000001000101",
                    "000001001000",
                    "000001001001",
                    "000001001010",
                    "000001010000",
                    "000001010001",
                    "000001010010",
                    "000001010100",
                    "000001010101",
                    "000010000000",
                    "000010000001",
                    "000010000010",
                    "000010000100",
                    "000010000101",
                    "000010001000",
                    "000010001001",
                    "000010001010",
                    "000010010000",
                    "000010010001",
                    "000010010010",
                    "000010010100",
                    "000010010101",
                    "000010100000",
                    "000010100001",
                    "000010100010",
                    "000010100100",
                    "000010100101",
                    "000010101000",
                    "000010101001",
                    "000010101010",
                    "000100000000",
                    "000100000001",
                    "000100000010",
                    "000100000100",
                    "000100000101",
                    "000100001000",
                    "000100001001",
                    "000100001010",
                    "000100010000",
                    "000100010001",
                    "000100010010",
                    "000100010100",
                    "000100010101",
                    "000100100000",
                    "000100100001",
                    "000100100010",
                    "000100100100",
                    "000100100101",
                    "000100101000",
                    "000100101001",
                    "000100101010",
                    "000101000000",
                    "000101000001",
                    "000101000010",
                    "000101000100",
                    "000101000101",
                    "000101001000",
                    "000101001001",
                    "000101001010",
                    "000101010000",
                    "000101010001",
                    "000101010010",
                    "000101010100",
                    "000101010101",
                    "001000000000",
                    "001000000001",
                    "001000000010",
                    "001000000100",
                    "001000000101",
                    "001000001000",
                    "001000001001",
                    "001000001010",
                    "001000010000",
                    "001000010001",
                    "001000010010",
                    "001000010100",
                    "001000010101",
                    "001000100000",
                    "001000100001",
                    "001000100010",
                    "001000100100",
                    "001000100101",
                    "001000101000",
                    "001000101001",
                    "001000101010",
                    "001001000000",
                    "001001000001",
                    "001001000010",
                    "001001000100",
                    "001001000101",
                    "001001001000",
                    "001001001001",
                    "001001001010",
                    "001001010000",
                    "001001010001",
                    "001001010010",
                    "001001010100",
                    "001001010101",
                    "001010000000",
                    "001010000001",
                    "001010000010",
                    "001010000100",
                    "001010000101",
                    "001010001000",
                    "001010001001",
                    "001010001010",
                    "001010010000",
                    "001010010001",
                    "001010010010",
                    "001010010100",
                    "001010010101",
                    "001010100000",
                    "001010100001",
                    "001010100010",
                    "001010100100",
                    "001010100101",
                    "001010101000",
                    "001010101001",
                    "001010101010",
                    "010000000000",
                    "010000000001",
                    "010000000010",
                    "010000000100",
                    "010000000101",
                    "010000001000",
                    "010000001001",
                    "010000001010",
                    "010000010000",
                    "010000010001",
                    "010000010010",
                    "010000010100",
                    "010000010101",
                    "010000100000",
                    "010000100001",
                    "010000100010",
                    "010000100100",
                    "010000100101",
                    "010000101000",
                    "010000101001",
                    "010000101010",
                    "010001000000",
                    "010001000001",
                    "010001000010",
                    "010001000100",
                    "010001000101",
                    "010001001000",
                    "010001001001",
                    "010001001010",
                    "010001010000",
                    "010001010001",
                    "010001010010",
                    "010001010100",
                    "010001010101",
                    "010010000000",
                    "010010000001",
                    "010010000010",
                    "010010000100",
                    "010010000101",
                    "010010001000",
                    "010010001001",
                    "010010001010",
                    "010010010000",
                    "010010010001",
                    "010010010010",
                    "010010010100",
                    "010010010101",
                    "010010100000",
                    "010010100001",
                    "010010100010",
                    "010010100100",
                    "010010100101",
                    "010010101000",
                    "010010101001",
                    "010010101010",
                    "010100000000",
                    "010100000001",
                    "010100000010",
                    "010100000100",
                    "010100000101",
                    "010100001000",
                    "010100001001",
                    "010100001010",
                    "010100010000",
                    "010100010001",
                    "010100010010",
                    "010100010100",
                    "010100010101",
                    "010100100000",
                    "010100100001",
                    "010100100010",
                    "010100100100",
                    "010100100101",
                    "010100101000",
                    "010100101001",
                    "010100101010",
                    "010101000000",
                    "010101000001",
                    "010101000010",
                    "010101000100",
                    "010101000101",
                    "010101001000",
                    "010101001001",
                    "010101001010",
                    "010101010000",
                    "010101010001",
                    "010101010010",
                    "010101010100",
                    "010101010101",
                    "100000000000",
                    "100000000001",
                    "100000000010",
                    "100000000100",
                    "100000000101",
                    "100000001000",
                    "100000001001",
                    "100000001010",
                    "100000010000",
                    "100000010001",
                    "100000010010",
                    "100000010100",
                    "100000010101",
                    "100000100000",
                    "100000100001",
                    "100000100010",
                    "100000100100",
                    "100000100101",
                    "100000101000",
                    "100000101001",
                    "100000101010",
                    "100001000000",
                    "100001000001",
                    "100001000010",
                    "100001000100",
                    "100001000101",
                    "100001001000",
                    "100001001001",
                    "100001001010",
                    "100001010000",
                    "100001010001",
                    "100001010010",
                    "100001010100",
                    "100001010101",
                    "100010000000",
                    "100010000001",
                    "100010000010",
                    "100010000100",
                    "100010000101",
                    "100010001000",
                    "100010001001",
                    "100010001010",
                    "100010010000",
                    "100010010001",
                    "100010010010",
                    "100010010100",
                    "100010010101",
                    "100010100000",
                    "100010100001",
                    "100010100010",
                    "100010100100",
                    "100010100101",
                    "100010101000",
                    "100010101001",
                    "100010101010",
                    "100100000000",
                    "100100000001",
                    "100100000010",
                    "100100000100",
                    "100100000101",
                    "100100001000",
                    "100100001001",
                    "100100001010",
                    "100100010000",
                    "100100010001",
                    "100100010010",
                    "100100010100",
                    "100100010101",
                    "100100100000",
                    "100100100001",
                    "100100100010",
                    "100100100100",
                    "100100100101",
                    "100100101000",
                    "100100101001",
                    "100100101010",
                    "100101000000",
                    "100101000001",
                    "100101000010",
                    "100101000100",
                    "100101000101",
                    "100101001000",
                    "100101001001",
                    "100101001010",
                    "100101010000",
                    "100101010001",
                    "100101010010",
                    "100101010100",
                    "100101010101",
                    "101000000000",
                    "101000000001",
                    "101000000010",
                    "101000000100",
                    "101000000101",
                    "101000001000",
                    "101000001001",
                    "101000001010",
                    "101000010000",
                    "101000010001",
                    "101000010010",
                    "101000010100",
                    "101000010101",
                    "101000100000",
                    "101000100001",
                    "101000100010",
                    "101000100100",
                    "101000100101",
                    "101000101000",
                    "101000101001",
                    "101000101010",
                    "101001000000",
                    "101001000001",
                    "101001000010",
                    "101001000100",
                    "101001000101",
                    "101001001000",
                    "101001001001",
                    "101001001010",
                    "101001010000",
                    "101001010001",
                    "101001010010",
                    "101001010100",
                    "101001010101",
                    "101010000000",
                    "101010000001",
                    "101010000010",
                    "101010000100",
                    "101010000101",
                    "101010001000",
                    "101010001001",
                    "101010001010",
                    "101010010000",
                    "101010010001",
                    "101010010010",
                    "101010010100",
                    "101010010101",
                    "101010100000",
                    "101010100001",
                    "101010100010",
                    "101010100100",
                    "101010100101",
                    "101010101000",
                    "101010101001",
                    "101010101010"
                ],
                binary_strings_with_nonrepeating_ones_of_length_sorted(12)
            )
        }
    }

    mod friends {
        use super::super::super::recursion::party_pairings;

        #[test]
        fn it_works_with_0_producing_1() {
            assert_eq!(1, party_pairings(0));
        }

        #[test]
        fn it_works_with_1_producing_1() {
            assert_eq!(1, party_pairings(1));
        }

        #[test]
        fn it_works_with_2_producing_2() {
            assert_eq!(2, party_pairings(2));
        }

        #[test]
        fn it_works_with_3_producing_4() {
            assert_eq!(4, party_pairings(3));
        }

        #[test]
        fn it_works_with_4_producing_10() {
            assert_eq!(10, party_pairings(4));
        }

        #[test]
        fn it_works_with_5_producing_26() {
            assert_eq!(26, party_pairings(5));
        }
    }

    mod divide_conquer {

        mod sort {

            mod merge_sort {
                use super::super::super::super::super::divide_conquer::sort::merge_sort;

                #[test]
                fn it_works_with_empty_array() {
                    let mut data = [0_u32; 0];
                    merge_sort(&mut data);
                    assert_eq!([0_u32; 0], data);
                }

                #[test]
                fn it_works_with_length_1_array() {
                    let mut data = [13];
                    merge_sort(&mut data);
                    assert_eq!([13], data);
                }

                #[test]
                fn it_works_with_length_2_array() {
                    let mut data = [7, 3];
                    merge_sort(&mut data);
                    assert_eq!([3, 7], data);
                }

                #[test]
                fn it_works_with_length_3_array() {
                    let mut data = [7, 3, 5];
                    merge_sort(&mut data);
                    assert_eq!([3, 5, 7], data);
                }

                #[test]
                fn it_works_with_this_array() {
                    let mut data = [7, 12, 9, 6, 1, 13, 7, 10, 31, 2, 5, 3, 99, 37, 26, 14];
                    merge_sort(&mut data);
                    assert_eq!(
                        [1, 2, 3, 5, 6, 7, 7, 9, 10, 12, 13, 14, 26, 31, 37, 99],
                        data
                    );
                }
            }

            mod quick_sort {
                use super::super::super::super::super::divide_conquer::sort::quick_sort;

                #[test]
                fn it_works_with_empty_array() {
                    let mut data = [0_u32; 0];
                    quick_sort(&mut data);
                    assert_eq!([0_u32; 0], data);
                }

                #[test]
                fn it_works_with_length_1_array() {
                    let mut data = [13];
                    quick_sort(&mut data);
                    assert_eq!([13], data);
                }

                #[test]
                fn it_works_with_length_2_array() {
                    let mut data = [7, 3];
                    quick_sort(&mut data);
                    assert_eq!([3, 7], data);
                }

                #[test]
                fn it_works_with_length_3_array() {
                    let mut data = [7, 3, 5];
                    quick_sort(&mut data);
                    assert_eq!([3, 5, 7], data);
                }

                #[test]
                fn it_works_with_this_array() {
                    let mut data = [7, 12, 9, 6, 1, 13, 7, 10, 31, 2, 5, 3, 99, 37, 26, 14];
                    quick_sort(&mut data);
                    assert_eq!(
                        [1, 2, 3, 5, 6, 7, 7, 9, 10, 12, 13, 14, 26, 31, 37, 99],
                        data
                    );
                }
            }

            mod merge_sort_2d {
                use super::super::super::super::super::divide_conquer::sort::merge_sort_2d;
                use super::super::super::super::super::print_2d::Vec2d;

                #[test]
                fn it_works_on_a_3_by_3_array() {
                    let expected = Vec2d::from(vec![
                        vec![1, 4, 11],   //
                        vec![9, 15, 18],  //
                        vec![13, 20, 23], //
                    ]);

                    let mut data = Vec2d::from(vec![
                        vec![18, 9, 11],  //
                        vec![1, 4, 15],   //
                        vec![13, 23, 20], //
                    ]);
                    merge_sort_2d(&mut data);

                    assert_eq! {
                        expected,
                        data
                    }
                }

                #[test]
                fn it_works_on_a_4_by_4_array() {
                    let expected = Vec2d::from(vec![
                        vec![1, 8, 16, 18],   //
                        vec![4, 13, 19, 23],  //
                        vec![11, 15, 20, 28], //
                        vec![24, 25, 26, 30], //
                    ]);

                    let mut data = Vec2d::from(vec![
                        vec![18, 4, 16, 8],   //
                        vec![23, 13, 20, 11], //
                        vec![28, 24, 26, 25], //
                        vec![1, 30, 15, 19],  //
                    ]);
                    merge_sort_2d(&mut data);

                    assert_eq! {
                        expected,
                        data
                    }
                }
            }
        }

        mod search {

            mod binary_search {
                use super::super::super::super::super::divide_conquer::search::binary_search;

                #[test]
                fn it_works_with_empty_array() {
                    assert_eq!(None, binary_search(&5, &[0_u32; 0]));
                }

                #[test]
                fn it_works_with_a_1_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(Some(0), binary_search(&5, &[5]));
                }

                #[test]
                fn it_works_with_a_1_element_array_where_the_searched_for_element_is_not_present() {
                    assert_eq!(None, binary_search(&5, &[6]));
                }

                #[test]
                fn it_works_with_a_2_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(Some(0), binary_search(&5, &[5, 6]));
                }

                #[test]
                fn it_works_with_a_2_element_array_where_the_searched_for_element_is_not_present() {
                    assert_eq!(None, binary_search(&5, &[6, 7]));
                }

                #[test]
                fn it_works_with_a_3_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(Some(2), binary_search(&12, &[5, 6, 12]));
                }

                #[test]
                fn it_works_with_a_3_element_array_where_the_searched_for_element_is_not_present() {
                    assert_eq!(None, binary_search(&8, &[6, 7, 13]));
                }

                #[test]
                fn it_works_with_a_12_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(
                        Some(5),
                        binary_search(&31, &[5, 6, 12, 17, 28, 31, 45, 56, 78, 92, 101, 203])
                    );
                    assert_eq!(
                        Some(9),
                        binary_search(&92, &[5, 6, 12, 17, 28, 31, 45, 56, 78, 92, 101, 203])
                    );
                    assert_eq!(
                        Some(3),
                        binary_search(&17, &[5, 6, 12, 17, 28, 31, 45, 56, 78, 92, 101, 203])
                    );
                }

                #[test]
                fn it_works_with_a_12_element_array_where_the_searched_for_element_is_not_present()
                {
                    assert_eq!(
                        None,
                        binary_search(&81, &[5, 6, 12, 17, 28, 31, 45, 56, 78, 92, 101, 203])
                    );
                }
            }

            mod rotated_binary_search {
                use super::super::super::super::super::divide_conquer::search::rotated_binary_search;

                #[test]
                fn it_works_with_empty_array() {
                    assert_eq!(None, rotated_binary_search(&5, &[0_u32; 0]));
                }

                #[test]
                fn it_works_with_a_1_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(Some(0), rotated_binary_search(&5, &[5]));
                }

                #[test]
                fn it_works_with_a_1_element_array_where_the_searched_for_element_is_not_present() {
                    assert_eq!(None, rotated_binary_search(&5, &[6]));
                }

                #[test]
                fn it_works_with_a_2_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(Some(1), rotated_binary_search(&5, &[6, 5]));
                }

                #[test]
                fn it_works_with_a_2_element_array_where_the_searched_for_element_is_not_present() {
                    assert_eq!(None, rotated_binary_search(&5, &[7, 6]));
                }

                #[test]
                fn it_works_with_a_3_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(Some(0), rotated_binary_search(&12, &[12, 5, 6]));
                }

                #[test]
                fn it_works_with_a_3_element_array_where_the_searched_for_element_is_not_present() {
                    assert_eq!(None, rotated_binary_search(&8, &[7, 13, 6]));
                }

                #[test]
                fn it_works_with_a_12_element_array_where_the_searched_for_element_is_present() {
                    assert_eq!(
                        Some(9),
                        rotated_binary_search(
                            &31,
                            &[78, 92, 101, 203, 5, 6, 12, 17, 28, 31, 45, 56,]
                        )
                    );
                    assert_eq!(
                        Some(2),
                        rotated_binary_search(
                            &92,
                            &[56, 78, 92, 101, 203, 5, 6, 12, 17, 28, 31, 45,]
                        )
                    );
                    assert_eq!(
                        Some(8),
                        rotated_binary_search(
                            &17,
                            &[56, 78, 92, 101, 203, 5, 6, 12, 17, 28, 31, 45,]
                        )
                    );
                }

                #[test]
                fn it_works_with_a_12_element_array_where_the_searched_for_element_is_not_present()
                {
                    assert_eq!(
                        None,
                        rotated_binary_search(
                            &81,
                            &[56, 78, 92, 101, 203, 5, 6, 12, 17, 28, 31, 45,]
                        )
                    );
                }
            }
        }
    }
}

mod backtracking {

    mod permutations {
        use super::super::super::backtracking::permutations::permutations;

        #[test]
        fn it_works() {
            let expected = vec![
                vec!['a', 'b', 'c'], //
                vec!['a', 'c', 'b'], //
                vec!['b', 'a', 'c'], //
                vec!['b', 'c', 'a'], //
                vec!['c', 'b', 'a'], //
                vec!['c', 'a', 'b'], //
            ];

            let mut input = vec!['a', 'b', 'c'];
            let actual = permutations(&mut input);

            assert_eq!(expected, actual);
        }
    }

    mod n_queens {

        mod find_a_solution {
            use super::super::super::super::backtracking::n_queens::n_queen_find_a_solution;

            #[test]
            fn it_works_for_0() {
                assert_eq!(Some(Vec::new()), n_queen_find_a_solution(0));
            }

            #[test]
            fn it_works_for_1() {
                assert_eq!(Some(vec![(0, 0)]), n_queen_find_a_solution(1));
            }

            #[test]
            fn it_works_for_2() {
                assert_eq!(None, n_queen_find_a_solution(2));
            }

            #[test]
            fn it_works_for_3() {
                assert_eq!(None, n_queen_find_a_solution(3));
            }

            #[test]
            fn it_works_for_4() {
                assert_eq!(
                    Some(vec![
                        (1, 0), //
                        (3, 1), //
                        (0, 2), //
                        (2, 3), //
                    ]),
                    n_queen_find_a_solution(4)
                );
            }
        }

        mod find_all_solutions {
            use super::super::super::super::backtracking::n_queens::n_queen_find_all_solutions;

            #[test]
            fn it_works_for_0() {
                assert_eq!(Some(vec![Vec::new()]), n_queen_find_all_solutions(0));
            }

            #[test]
            fn it_works_for_1() {
                assert_eq!(Some(vec![vec![(0, 0)]]), n_queen_find_all_solutions(1));
            }

            #[test]
            fn it_works_for_2() {
                assert_eq!(None, n_queen_find_all_solutions(2));
            }

            #[test]
            fn it_works_for_3() {
                assert_eq!(None, n_queen_find_all_solutions(3));
            }

            #[test]
            fn it_works_for_4() {
                assert_eq!(
                    Some(vec![
                        vec![
                            (1, 0), //
                            (3, 1), //
                            (0, 2), //
                            (2, 3), //
                        ],
                        vec![
                            (2, 0), //
                            (0, 1), //
                            (3, 2), //
                            (1, 3)  //
                        ]
                    ]),
                    n_queen_find_all_solutions(4)
                );
            }
        }
    }

    mod grid_ways {

        mod count_paths_to_end {
            use super::super::super::super::backtracking::grid_ways::count_paths_to_end;

            #[test]
            fn it_works_with_a_0_by_0_grid() {
                assert_eq!(0, count_paths_to_end(0, 0));
            }

            #[test]
            fn it_works_with_a_1_by_1_grid() {
                assert_eq!(1, count_paths_to_end(1, 1));
            }

            #[test]
            fn it_works_with_a_2_by_2_grid() {
                assert_eq!(2, count_paths_to_end(2, 2));
            }

            #[test]
            fn it_works_with_a_10_by_0_grid() {
                assert_eq!(0, count_paths_to_end(10, 0));
            }

            #[test]
            fn it_works_with_a_0_by_10_grid() {
                assert_eq!(0, count_paths_to_end(0, 10));
            }

            #[test]
            fn it_works_with_a_1_by_10_grid() {
                assert_eq!(1, count_paths_to_end(1, 10));
            }

            #[test]
            fn it_works_with_a_2_by_4_grid() {
                assert_eq!(4, count_paths_to_end(2, 4));
            }

            #[test]
            fn it_works_with_a_3_by_3_grid() {
                assert_eq!(6, count_paths_to_end(3, 3));
            }

            #[test]
            fn it_works_with_a_4_by_4_grid() {
                assert_eq!(20, count_paths_to_end(4, 4));
            }
        }

        mod count_paths_to_end_fast {
            use super::super::super::super::backtracking::grid_ways::count_paths_to_end_fast;

            #[test]
            fn it_works_with_a_0_by_0_grid() {
                assert_eq!(0, count_paths_to_end_fast(0, 0));
            }

            #[test]
            fn it_works_with_a_1_by_1_grid() {
                assert_eq!(1, count_paths_to_end_fast(1, 1));
            }

            #[test]
            fn it_works_with_a_2_by_2_grid() {
                assert_eq!(2, count_paths_to_end_fast(2, 2));
            }

            #[test]
            fn it_works_with_a_10_by_0_grid() {
                assert_eq!(0, count_paths_to_end_fast(10, 0));
            }

            #[test]
            fn it_works_with_a_0_by_10_grid() {
                assert_eq!(0, count_paths_to_end_fast(0, 10));
            }

            #[test]
            fn it_works_with_a_1_by_10_grid() {
                assert_eq!(1, count_paths_to_end_fast(1, 10));
            }

            #[test]
            fn it_works_with_a_2_by_4_grid() {
                assert_eq!(4, count_paths_to_end_fast(2, 4));
            }

            #[test]
            fn it_works_with_a_3_by_3_grid() {
                assert_eq!(6, count_paths_to_end_fast(3, 3));
            }

            #[test]
            fn it_works_with_a_4_by_4_grid() {
                assert_eq!(20, count_paths_to_end_fast(4, 4));
            }

            #[test]
            fn it_works_with_a_20_by_20_grid() {
                assert_eq!(35_345_263_800, count_paths_to_end_fast(20, 20));
            }

            #[test]
            fn it_works_with_a_30_by_30_grid() {
                assert_eq!(30_067_266_499_541_040, count_paths_to_end_fast(30, 30));
            }

            #[test]
            fn it_works_with_a_40_by_40_grid() {
                assert_eq!(
                    27_217_014_869_199_032_015_600,
                    count_paths_to_end_fast(40, 40)
                );
            }

            #[test]
            fn it_works_with_a_50_by_50_grid() {
                assert_eq!(
                    25_477_612_258_980_856_902_730_428_600,
                    count_paths_to_end_fast(50, 50)
                );
            }

            #[test]
            fn it_works_with_a_60_by_60_grid() {
                assert_eq!(
                    24_356_699_707_654_619_143_838_606_602_026_720,
                    count_paths_to_end_fast(60, 60)
                );
            }

            #[test]
            #[should_panic(expected = "attempt to add with overflow")]
            fn it_panics_on_overflow_with_a_100_by_100_grid() {
                count_paths_to_end_fast(100, 100);
            }
        }
    }

    mod sudoku {
        use super::super::super::backtracking::sudoku::solve_sudoku;
        use super::super::super::print_2d::Vec2d;

        #[test]
        fn it_works() {
            let expected_solution = Vec2d::from(vec![
                vec![5, 3, 4, 6, 7, 8, 9, 1, 2], //
                vec![6, 7, 2, 1, 9, 5, 3, 4, 8], //
                vec![1, 9, 8, 3, 4, 2, 5, 6, 7], //
                vec![8, 5, 9, 7, 6, 1, 4, 2, 3], //
                vec![4, 2, 6, 8, 5, 3, 7, 9, 1], //
                vec![7, 1, 3, 9, 2, 4, 8, 5, 6], //
                vec![9, 6, 1, 5, 3, 7, 2, 8, 4], //
                vec![2, 8, 7, 4, 1, 9, 6, 3, 5], //
                vec![3, 4, 5, 2, 8, 6, 1, 7, 9], //
            ]);

            let mut board = Vec2d::from(vec![
                vec![5, 3, 0, 0, 7, 0, 0, 0, 0], //
                vec![6, 0, 0, 1, 9, 5, 0, 0, 0], //
                vec![0, 9, 8, 0, 0, 0, 0, 6, 0], //
                vec![8, 0, 0, 0, 6, 0, 0, 0, 3], //
                vec![4, 0, 0, 8, 0, 3, 0, 0, 1], //
                vec![7, 0, 0, 0, 2, 0, 0, 0, 6], //
                vec![0, 6, 0, 0, 0, 0, 2, 8, 0], //
                vec![0, 0, 0, 4, 1, 9, 0, 0, 5], //
                vec![0, 0, 0, 0, 8, 0, 0, 7, 9], //
            ]);
            assert!(solve_sudoku(&mut board), "board has no solution");

            assert_eq!(
                expected_solution, board,
                "board solution does not match expected board solution"
            );
        }
    }

    mod rat_maze {
        use super::super::super::backtracking::rat_maze::find_path_to_end;
        use super::super::super::print_2d::Vec2d;

        #[test]
        fn it_works() {
            let expected = vec![
                (0, 0), //
                (0, 1), //
                (1, 1), //
                (1, 2), //
                (1, 3), //
                (2, 3), //
                (2, 4), //
                (3, 4), //
            ];

            let maze = Vec2d::from(vec![
                vec![false, true, false, false], //
                vec![false, false, false, true], //
                vec![true, false, true, false],  //
                vec![true, false, false, true],  //
                vec![true, true, false, false],  //
            ]);

            assert_eq!(
                Some(expected),
                find_path_to_end(maze),
                "no solution or incorrect solution found for maze"
            );
        }
    }

    mod word_break {
        use std::collections::HashSet;

        use super::super::super::backtracking::word_break::break_into_sentences;

        #[test]
        fn it_works() {
            let dictionary = HashSet::from(["i", "like", "sam", "sung", "samsung", "mobile"]);
            let expected = vec!["i like sam sung mobile", "i like samsung mobile"];
            let input = "ilikesamsungmobile";

            assert_eq!(expected, break_into_sentences(input, dictionary));
        }
    }

    mod power_set {
        use std::collections::{HashSet, VecDeque};

        use super::super::super::backtracking::power_set::power_set;

        #[test]
        fn it_works() {
            let expected = HashSet::from([
                VecDeque::from([]),        //
                VecDeque::from([1]),       //
                VecDeque::from([1, 2]),    //
                VecDeque::from([1, 2, 2]), //
                VecDeque::from([2]),       //
                VecDeque::from([2, 2]),    //
            ]);

            let input = vec![1, 2, 2];

            assert_eq!(expected, power_set(&input));
        }
    }

    mod word_search {
        use super::super::super::print_2d::Vec2d;

        use super::super::super::backtracking::word_search::find_word;

        #[test]
        fn it_works_with_a_simple_example() {
            assert!(find_word(
                &Vec2d::from(vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ]),
                "ABCCED"
            ))
        }

        #[test]
        fn it_works_with_a_more_complicated_example() {
            assert!(find_word(
                &Vec2d::from(vec![
                    vec!['A', 'B', 'C', 'E', 'G', 'L'],
                    vec!['S', 'M', 'C', 'S', 'T', 'P'],
                    vec!['A', 'O', 'E', 'M', 'O', 'R'],
                    vec!['C', 'M', 'U', 'P', 'H', 'Q'],
                    vec!['L', 'K', 'M', 'D', 'X', 'P'],
                    vec!['B', 'C', 'O', 'R', 'I', 'U'],
                    vec!['S', 'U', 'R', 'P', 'H', 'G'],
                    vec!['D', 'I', 'H', 'P', 'R', 'P'],
                    vec!['M', 'Y', 'Z', 'V', 'O', 'H'],
                    vec!['O', 'R', 'P', 'H', 'M', 'I'],
                ]),
                "MORPHIUS"
            ))
        }
    }
}

mod linked_list {

    use super::super::linked_list::{
        cursor::{Cursor, CursorStart, MutCursor},
        LinkedList,
    };

    #[test]
    fn it_can_be_instantiated_by_new_or_default() {
        let _ll1 = LinkedList::<u32>::new();
        let _ll2 = LinkedList::<String>::default();
    }

    #[test]
    fn it_returns_proper_values_for_len_and_is_empty_for_an_empty_list() {
        let ll = LinkedList::<u32>::new();
        assert_eq!(0, ll.len());
        assert!(ll.is_empty());
        assert_eq!(None, ll.peek_head());
        assert_eq!(None, ll.peek_tail());
    }

    #[test]
    fn it_allows_a_value_to_be_pushed_to_head_and_peeked_at_head_and_tail_and_len_and_is_empty_are_correct(
    ) {
        let mut ll = LinkedList::<u32>::new();
        ll.push_head(u32::MAX);
        assert_eq!(Some(u32::MAX), ll.peek_head().copied());
        assert_eq!(Some(u32::MAX), ll.peek_tail().copied());
        assert_eq!(1, ll.len());
        assert!(!ll.is_empty());
    }

    #[test]
    fn it_allows_a_value_to_be_pushed_to_tail_and_peeked_at_head_and_tail_and_len_and_is_empty_are_correct(
    ) {
        let mut ll = LinkedList::<u32>::new();
        ll.push_tail(u32::MAX);
        assert_eq!(Some(u32::MAX), ll.peek_head().copied());
        assert_eq!(Some(u32::MAX), ll.peek_tail().copied());
        assert_eq!(1, ll.len());
        assert!(!ll.is_empty());
    }

    #[test]
    fn it_allows_a_value_to_be_pushed_to_head_and_popped_at_head_and_tail() {
        let mut ll = LinkedList::<u32>::new();

        ll.push_head(u32::MAX);
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        assert_eq!(Some(u32::MAX), ll.pop_head());
        assert!(ll.is_empty());
        assert_eq!(0, ll.len());

        ll.push_head(0);
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        assert_eq!(Some(0), ll.pop_tail());
        assert!(ll.is_empty());
        assert_eq!(0, ll.len());
    }

    #[test]
    fn it_allows_a_value_to_be_pushed_to_tail_and_popped_at_head_and_tail() {
        let mut ll = LinkedList::<u32>::new();

        ll.push_tail(u32::MAX);
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        assert_eq!(Some(u32::MAX), ll.pop_head());
        assert!(ll.is_empty());
        assert_eq!(0, ll.len());

        ll.push_tail(0);
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        assert_eq!(Some(0), ll.pop_tail());
        assert!(ll.is_empty());
        assert_eq!(0, ll.len());
    }

    #[test]
    fn it_allows_multiple_pushes_peeks_pops_and_it_accurately_tracks_everything() {
        let mut ll = LinkedList::<&str>::new();

        assert!(ll.is_empty());
        assert_eq!(0, ll.len());
        assert_eq!(None, ll.peek_head());
        assert_eq!(None, ll.peek_tail());
        assert_eq!(None, ll.pop_head());
        assert_eq!(None, ll.pop_tail());
        assert!(ll.is_empty());
        assert_eq!(0, ll.len());

        ll.push_head("Hello");
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        assert_eq!(Some("Hello"), ll.peek_head().copied());
        assert_eq!(Some("Hello"), ll.peek_tail().copied());
        ll.push_tail("World");
        assert!(!ll.is_empty());
        assert_eq!(2, ll.len());
        assert_eq!(Some("Hello"), ll.peek_head().copied());
        assert_eq!(Some("World"), ll.peek_tail().copied());
        assert_eq!(Some("World"), ll.pop_tail());
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        assert_eq!(Some("Hello"), ll.peek_head().copied());
        assert_eq!(Some("Hello"), ll.peek_tail().copied());
        assert_eq!(Some("Hello"), ll.pop_tail());
        assert_eq!(None, ll.peek_head());
        assert_eq!(None, ll.peek_tail());
        assert!(ll.is_empty());
        assert_eq!(0, ll.len());

        ll.push_head("Hello");
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        assert_eq!(Some("Hello"), ll.peek_head().copied());
        assert_eq!(Some("Hello"), ll.peek_tail().copied());
        ll.push_tail("World");
        assert!(!ll.is_empty());
        assert_eq!(2, ll.len());
        assert_eq!(Some("Hello"), ll.peek_head().copied());
        assert_eq!(Some("World"), ll.peek_tail().copied());
        ll.push_head("Awesome");
        assert!(!ll.is_empty());
        assert_eq!(3, ll.len());
        assert_eq!(Some("Awesome"), ll.peek_head().copied());
        assert_eq!(Some("World"), ll.peek_tail().copied());
        assert_eq!(Some("World"), ll.pop_tail());
        assert!(!ll.is_empty());
        assert_eq!(2, ll.len());
        assert_eq!(Some("Awesome"), ll.peek_head().copied());
        assert_eq!(Some("Hello"), ll.peek_tail().copied());
        assert_eq!(Some("Hello"), ll.pop_tail());
        assert_eq!(Some("Awesome"), ll.peek_head().copied());
        assert_eq!(Some("Awesome"), ll.peek_tail().copied());
        assert!(!ll.is_empty());
        assert_eq!(1, ll.len());
        ll.push_tail("Isn't");
        ll.push_head("is");
        ll.push_tail("it");
        ll.push_head("This");
        assert_eq!(Some("This"), ll.peek_head().copied());
        assert_eq!(Some("it"), ll.peek_tail().copied());
        assert_eq!(Some("This"), ll.pop_head());
        assert_eq!(Some("is"), ll.pop_head());
        assert_eq!(Some("Awesome"), ll.pop_head());
        assert_eq!(Some("Isn't"), ll.pop_head());
        assert_eq!(Some("it"), ll.pop_head());
        assert!(ll.is_empty());
        assert_eq!(0, ll.len());
        assert_eq!(None, ll.peek_head());
        assert_eq!(None, ll.peek_tail());
        assert_eq!(None, ll.pop_head());
        assert_eq!(None, ll.pop_tail());
    }

    #[test]
    fn it_iterates_from_front_to_back_yielding_shared_references_to_the_contained_values() {
        let mut ll = LinkedList::new();

        ll.push_tail("This");
        ll.push_tail("is");
        ll.push_tail("a");
        ll.push_tail("test");
        ll.push_tail("of");
        ll.push_tail("the");
        ll.push_tail("LinkedList");
        ll.push_tail("to");
        ll.push_tail("ensure");
        ll.push_tail("the");
        ll.push_tail("iter");
        ll.push_tail("method");
        ll.push_tail("yields");
        ll.push_tail("a");
        ll.push_tail("valid");
        ll.push_tail("iterator");
        ll.push_tail("that");
        ll.push_tail("will");
        ll.push_tail("iterate");
        ll.push_tail("head");
        ll.push_tail("to");
        ll.push_tail("tail");
        ll.push_tail("correctly.");
        ll.push_head("ATTENTION!!!");

        let mut result = String::new();
        for value in ll.iter() {
            result = result + value;
            result.push(' ');
        }

        assert_eq!(
            "ATTENTION!!! This is a test of the LinkedList to ensure the iter method \
                 yields a valid iterator that will iterate head to tail correctly. ",
            result.as_str()
        );
    }

    #[test]
    fn it_iterates_from_back_to_front_yielding_shared_references_to_the_contained_values() {
        let mut ll = LinkedList::new();

        ll.push_tail("This");
        ll.push_tail("is");
        ll.push_tail("a");
        ll.push_tail("test");
        ll.push_tail("of");
        ll.push_tail("the");
        ll.push_tail("LinkedList");
        ll.push_tail("to");
        ll.push_tail("ensure");
        ll.push_tail("the");
        ll.push_tail("iter");
        ll.push_tail("method");
        ll.push_tail("yields");
        ll.push_tail("a");
        ll.push_tail("valid");
        ll.push_tail("iterator");
        ll.push_tail("that");
        ll.push_tail("will");
        ll.push_tail("iterate");
        ll.push_tail("tail");
        ll.push_tail("to");
        ll.push_tail("head");
        ll.push_tail("correctly.");
        ll.push_head("ATTENTION!!!");

        let mut result = String::new();
        for value in ll.iter().rev() {
            result = result + value;
            result.push(' ');
        }

        assert_eq!(
            "correctly. head to tail iterate will that iterator valid a yields method iter \
                 the ensure to LinkedList the of test a is This ATTENTION!!! ",
            result.as_str()
        );
    }

    #[test]
    fn it_iterates_from_front_to_back_yielding_mutable_references_to_the_contained_values() {
        let mut ll = LinkedList::new();

        ll.push_tail("This");
        ll.push_tail("is");
        ll.push_tail("a");
        ll.push_tail("test");
        ll.push_tail("of");
        ll.push_tail("the");
        ll.push_tail("LinkedList");
        ll.push_tail("to");
        ll.push_tail("ensure");
        ll.push_tail("the");
        ll.push_tail("iter");
        ll.push_tail("method");
        ll.push_tail("yields");
        ll.push_tail("a");
        ll.push_tail("valid");
        ll.push_tail("iterator");
        ll.push_tail("that");
        ll.push_tail("will");
        ll.push_tail("iterate");
        ll.push_tail("head");
        ll.push_tail("to");
        ll.push_tail("tail");
        ll.push_tail("correctly.");
        ll.push_head("ATTENTION!!!");

        for value in ll.iter_mut() {
            *value = "XXX";
        }

        let mut result = String::new();
        for value in ll.iter() {
            result = result + value;
            result.push(' ');
        }

        assert_eq!(
            "XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX \
                 XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX ",
            result.as_str()
        );
    }

    #[test]
    fn it_iterates_from_back_to_front_yielding_mutable_references_to_the_contained_values() {
        let mut ll = LinkedList::new();

        ll.push_tail("This");
        ll.push_tail("is");
        ll.push_tail("a");
        ll.push_tail("test");
        ll.push_tail("of");
        ll.push_tail("the");
        ll.push_tail("LinkedList");
        ll.push_tail("to");
        ll.push_tail("ensure");
        ll.push_tail("the");
        ll.push_tail("iter");
        ll.push_tail("method");
        ll.push_tail("yields");
        ll.push_tail("a");
        ll.push_tail("valid");
        ll.push_tail("iterator");
        ll.push_tail("that");
        ll.push_tail("will");
        ll.push_tail("iterate");
        ll.push_tail("tail");
        ll.push_tail("to");
        ll.push_tail("head");
        ll.push_tail("correctly.");
        ll.push_head("ATTENTION!!!");

        for value in ll.iter_mut().rev() {
            *value = "YYY";
        }

        let mut result = String::new();
        for value in ll.iter().rev() {
            result = result + value;
            result.push(' ');
        }

        assert_eq!(
            "YYY YYY YYY YYY YYY YYY YYY YYY YYY YYY YYY YYY YYY \
                 YYY YYY YYY YYY YYY YYY YYY YYY YYY YYY YYY ",
            result.as_str()
        );
    }

    #[test]
    fn it_allows_a_readonly_cursor_to_traverse_it_in_both_directions_and_to_examine_values() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!",
            "This",
            "is",
            "a",
            "test",
            "of",
            "the",
            "LinkedList",
            "cursor.",
        ];

        ll.push_tail(inputs[1]);
        ll.push_tail(inputs[2]);
        ll.push_tail(inputs[3]);
        ll.push_tail(inputs[4]);
        ll.push_tail(inputs[5]);
        ll.push_tail(inputs[6]);
        ll.push_tail(inputs[7]);
        ll.push_tail(inputs[8]);
        ll.push_head(inputs[0]);

        let mut cursor = ll.cursor(CursorStart::Head);
        assert!(!cursor.list_is_empty());
        assert_eq!(inputs.len(), cursor.list_len());
        assert!(cursor.on_item());
        assert_eq!(Some(0), cursor.at());

        for (i, input) in inputs.into_iter().enumerate() {
            assert_eq!(Some(i), cursor.at());
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(!cursor.list_is_empty());
            assert_eq!(inputs.len(), cursor.list_len());
            assert!(cursor.on_item());
            assert!(cursor.next().is_ok() || i == inputs.len() - 1);
        }

        assert!(!cursor.list_is_empty());
        assert_eq!(inputs.len(), cursor.list_len());
        assert!(!cursor.on_item());
        assert_eq!(None, cursor.at());

        assert!(cursor.seek_tail().is_ok());

        for (i, input) in inputs.into_iter().enumerate().rev() {
            assert_eq!(Some(i), cursor.at());
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(!cursor.list_is_empty());
            assert_eq!(inputs.len(), cursor.list_len());
            assert!(cursor.on_item());
            assert!(cursor.prev().is_ok() || i == 0);
        }

        assert!(!cursor.list_is_empty());
        assert_eq!(inputs.len(), cursor.list_len());
        assert!(!cursor.on_item());
        assert_eq!(None, cursor.at());

        assert!(cursor.seek_head().is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(0), cursor.at());
        assert_eq!(inputs[0], *cursor.peek().unwrap());
        assert!(cursor.seek_tail().is_ok());

        assert!(cursor.seek(0).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(0), cursor.at());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        assert!(cursor.seek_next(3).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(3), cursor.at());
        assert_eq!(inputs[3], *cursor.peek().unwrap());

        assert!(cursor.seek_prev(2).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(1), cursor.at());
        assert_eq!(inputs[1], *cursor.peek().unwrap());

        assert!(cursor.seek(inputs.len() - 1).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(inputs.len() - 1), cursor.at());
        assert_eq!(inputs[inputs.len() - 1], *cursor.peek().unwrap());

        assert!(cursor.seek_prev(2).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(inputs.len() - 3), cursor.at());
        assert_eq!(inputs[inputs.len() - 3], *cursor.peek().unwrap());

        assert_eq!(inputs[inputs.len() - 4], *cursor.peek_prev().unwrap());
        assert_eq!(inputs[inputs.len() - 2], *cursor.peek_next().unwrap());

        assert!(cursor.seek_tail().is_ok());
        assert_eq!(None, cursor.peek_next());

        assert!(cursor.seek_head().is_ok());
        assert_eq!(None, cursor.peek_prev());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_traverse_it_in_both_directions_and_to_examine_values() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!",
            "This",
            "is",
            "a",
            "test",
            "of",
            "the",
            "LinkedList",
            "cursor.",
        ];

        ll.push_tail(inputs[1]);
        ll.push_tail(inputs[2]);
        ll.push_tail(inputs[3]);
        ll.push_tail(inputs[4]);
        ll.push_tail(inputs[5]);
        ll.push_tail(inputs[6]);
        ll.push_tail(inputs[7]);
        ll.push_tail(inputs[8]);
        ll.push_head(inputs[0]);

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        assert!(!cursor.list_is_empty());
        assert_eq!(inputs.len(), cursor.list_len());
        assert!(cursor.on_item());
        assert_eq!(Some(0), cursor.at());

        for (i, input) in inputs.into_iter().enumerate() {
            assert_eq!(Some(i), cursor.at());
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(!cursor.list_is_empty());
            assert_eq!(inputs.len(), cursor.list_len());
            assert!(cursor.on_item());
            assert!(cursor.next().is_ok() || i == inputs.len() - 1);
        }

        assert!(!cursor.list_is_empty());
        assert_eq!(inputs.len(), cursor.list_len());
        assert!(!cursor.on_item());
        assert_eq!(None, cursor.at());

        assert!(cursor.seek_tail().is_ok());

        for (i, input) in inputs.into_iter().enumerate().rev() {
            assert_eq!(Some(i), cursor.at());
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(!cursor.list_is_empty());
            assert_eq!(inputs.len(), cursor.list_len());
            assert!(cursor.on_item());
            assert!(cursor.prev().is_ok() || i == 0);
        }

        assert!(!cursor.list_is_empty());
        assert_eq!(inputs.len(), cursor.list_len());
        assert!(!cursor.on_item());
        assert_eq!(None, cursor.at());

        assert!(cursor.seek_head().is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(0), cursor.at());
        assert_eq!(inputs[0], *cursor.peek().unwrap());
        assert!(cursor.seek_tail().is_ok());

        assert!(cursor.seek(0).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(0), cursor.at());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        assert!(cursor.seek_next(3).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(3), cursor.at());
        assert_eq!(inputs[3], *cursor.peek().unwrap());

        assert!(cursor.seek_prev(2).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(1), cursor.at());
        assert_eq!(inputs[1], *cursor.peek().unwrap());

        assert!(cursor.seek(inputs.len() - 1).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(inputs.len() - 1), cursor.at());
        assert_eq!(inputs[inputs.len() - 1], *cursor.peek().unwrap());

        assert!(cursor.seek_prev(2).is_ok());
        assert!(cursor.on_item());
        assert_eq!(Some(inputs.len() - 3), cursor.at());
        assert_eq!(inputs[inputs.len() - 3], *cursor.peek().unwrap());

        assert_eq!(inputs[inputs.len() - 4], *cursor.peek_prev().unwrap());
        assert_eq!(inputs[inputs.len() - 2], *cursor.peek_next().unwrap());

        assert!(cursor.seek_tail().is_ok());
        assert_eq!(None, cursor.peek_next());

        assert!(cursor.seek_head().is_ok());
        assert_eq!(None, cursor.peek_prev());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_insert_at_head() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!",
            "This",
            "is",
            "a",
            "test",
            "of",
            "the",
            "LinkedList",
            "cursor.",
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        assert!(cursor.list_is_empty());
        assert_eq!(0, cursor.list_len());
        assert_eq!(None, cursor.at());
        assert!(!cursor.on_item());

        cursor.insert_head(inputs[0]);
        assert!(!cursor.list_is_empty());
        assert_eq!(1, cursor.list_len());
        assert_eq!(None, cursor.at());
        assert!(!cursor.on_item());

        assert!(cursor.seek_head().is_ok());
        assert!(!cursor.list_is_empty());
        assert_eq!(1, cursor.list_len());
        assert_eq!(Some(0), cursor.at());
        assert!(cursor.on_item());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        assert!(cursor.seek_tail().is_ok());
        assert!(!cursor.list_is_empty());
        assert_eq!(1, cursor.list_len());
        assert_eq!(Some(0), cursor.at());
        assert!(cursor.on_item());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        let mut prev_head = *cursor.peek().unwrap();
        for i in (1..inputs.len()).rev() {
            cursor.insert_head(inputs[i]);
            assert_eq!(prev_head, *cursor.peek().unwrap());
            assert!(cursor.prev().is_ok());
            assert_eq!(inputs[i], *cursor.peek().unwrap());
            prev_head = *cursor.peek().unwrap();
        }

        assert!(cursor.seek_tail().is_ok());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        assert!(cursor.seek_head().is_ok());
        for input in inputs.into_iter().skip(1) {
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(cursor.next().is_ok());
        }
        assert_eq!(inputs[0], *cursor.peek().unwrap());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_insert_at_tail() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!",
            "This",
            "is",
            "a",
            "test",
            "of",
            "the",
            "LinkedList",
            "cursor.",
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Tail);
        assert!(cursor.list_is_empty());
        assert_eq!(0, cursor.list_len());
        assert_eq!(None, cursor.at());
        assert!(!cursor.on_item());

        cursor.insert_tail(inputs[0]);
        assert!(!cursor.list_is_empty());
        assert_eq!(1, cursor.list_len());
        assert_eq!(None, cursor.at());
        assert!(!cursor.on_item());

        assert!(cursor.seek_tail().is_ok());
        assert!(!cursor.list_is_empty());
        assert_eq!(1, cursor.list_len());
        assert_eq!(Some(0), cursor.at());
        assert!(cursor.on_item());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        assert!(cursor.seek_head().is_ok());
        assert!(!cursor.list_is_empty());
        assert_eq!(1, cursor.list_len());
        assert_eq!(Some(0), cursor.at());
        assert!(cursor.on_item());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        let mut prev_tail = *cursor.peek().unwrap();
        for input in inputs {
            cursor.insert_tail(input);
            assert_eq!(prev_tail, *cursor.peek().unwrap());
            assert!(cursor.next().is_ok());
            assert_eq!(input, *cursor.peek().unwrap());
            prev_tail = *cursor.peek().unwrap();
        }

        assert!(cursor.seek_head().is_ok());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        assert!(cursor.seek_tail().is_ok());
        for input in inputs.into_iter().skip(1).rev() {
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(cursor.prev().is_ok());
        }
        assert_eq!(inputs[0], *cursor.peek().unwrap());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_insert_after() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!",
            "This",
            "is",
            "a",
            "test",
            "of",
            "the",
            "LinkedList",
            "cursor.",
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        cursor.insert_head(inputs[0]);
        assert_eq!(None, cursor.at());
        assert_eq!(1, cursor.list_len());
        assert!(cursor.seek_head().is_ok());
        assert_eq!(Some(0), cursor.at());
        assert_eq!(1, cursor.list_len());

        for (i, input) in inputs.into_iter().skip(1).enumerate() {
            assert_eq!(Some(i), cursor.at());
            assert_eq!(1 + i, cursor.list_len());
            assert!(cursor.insert_after(input).is_ok());
            assert_eq!(Some(i), cursor.at());
            assert_eq!(2 + i, cursor.list_len());
            assert_eq!(input, *cursor.peek_next().unwrap());
            assert!(cursor.next().is_ok());
            assert_eq!(Some(i + 1), cursor.at());
            assert_eq!(2 + i, cursor.list_len());
            assert_eq!(input, *cursor.peek().unwrap());
        }

        assert!(cursor.seek_tail().is_ok());
        assert_eq!(inputs[inputs.len() - 1], *cursor.peek().unwrap());

        assert!(cursor.seek_head().is_ok());
        for (i, input) in inputs.into_iter().enumerate() {
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(cursor.next().is_ok() || i == inputs.len() - 1);
        }
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_insert_before() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!",
            "This",
            "is",
            "a",
            "test",
            "of",
            "the",
            "LinkedList",
            "cursor.",
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        cursor.insert_tail(inputs[0]);
        assert!(cursor.seek_tail().is_ok());
        assert_eq!(Some(0), cursor.at());
        assert_eq!(1, cursor.list_len());

        for (i, input) in inputs.into_iter().skip(1).enumerate() {
            assert_eq!(Some(0), cursor.at());
            assert_eq!(1 + i, cursor.list_len());
            assert!(cursor.insert_before(input).is_ok());
            assert_eq!(Some(1), cursor.at());
            assert_eq!(2 + i, cursor.list_len());
            assert_eq!(input, *cursor.peek_prev().unwrap());
            assert!(cursor.prev().is_ok());
            assert_eq!(Some(0), cursor.at());
            assert_eq!(2 + i, cursor.list_len());
            assert_eq!(input, *cursor.peek().unwrap());
        }

        assert!(cursor.seek_tail().is_ok());
        assert_eq!(inputs[0], *cursor.peek().unwrap());

        assert!(cursor.seek_tail().is_ok());
        for (i, input) in inputs.into_iter().enumerate() {
            assert_eq!(input, *cursor.peek().unwrap());
            assert!(cursor.prev().is_ok() || i == inputs.len() - 1);
        }
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_remove_a_value() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!",
            "This",
            "is",
            "a",
            "test",
            "of",
            "the",
            "LinkedList",
            "cursor.",
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        cursor.insert_head(inputs[0]);
        assert!(cursor.seek_head().is_ok());
        for input in inputs.into_iter().skip(1) {
            assert!(cursor.insert_after(input).is_ok());
            assert_eq!(input, *cursor.peek_next().unwrap());
            assert!(cursor.next().is_ok());
        }

        // Remove at head
        assert!(cursor.seek_head().is_ok());
        assert_eq!(Ok(inputs[0]), cursor.remove());
        assert_eq!(Some(inputs[1]), cursor.peek().copied());
        assert_eq!(None, cursor.peek_prev());
        assert_eq!(Some(inputs[2]), cursor.peek_next().copied());
        assert_eq!(Some(0), cursor.at());
        assert!(cursor.seek_head().is_ok());
        assert_eq!(Some(inputs[1]), cursor.peek().copied());

        // Remove at tail
        assert!(cursor.seek_tail().is_ok());
        assert_eq!(Ok(inputs[inputs.len() - 1]), cursor.remove());
        assert_eq!(Some(inputs[inputs.len() - 2]), cursor.peek().copied());
        assert_eq!(Some(inputs[inputs.len() - 3]), cursor.peek_prev().copied());
        assert_eq!(None, cursor.peek_next());
        assert_eq!(Some(inputs.len() - 3), cursor.at());
        assert!(cursor.seek_tail().is_ok());
        assert_eq!(Some(inputs[inputs.len() - 2]), cursor.peek().copied());

        // Remove in Middle
        assert!(cursor.seek_prev(3).is_ok());
        assert_eq!(Ok(inputs[4]), cursor.remove());
        assert_eq!(Some(inputs[5]), cursor.peek().copied());
        assert_eq!(Some(inputs[3]), cursor.peek_prev().copied());
        assert_eq!(Some(inputs[6]), cursor.peek_next().copied());
        assert_eq!(Some(inputs.len() - 6), cursor.at());

        // Remove All from Head
        assert!(cursor.seek_head().is_ok());
        for input_idx in [1, 2, 3, 5, 6, 7] {
            assert_eq!(Ok(inputs[input_idx]), cursor.remove());
        }

        assert!(cursor.list_is_empty());
        assert!(!cursor.on_item());
        assert_eq!(0, cursor.list_len());

        // Re-populate the List
        cursor.insert_head(inputs[0]);
        assert_eq!(None, cursor.at());
        assert!(cursor.seek_head().is_ok());
        assert_eq!(Some(0), cursor.at());
        for (i, input) in inputs.into_iter().skip(1).enumerate() {
            assert_eq!(Some(i), cursor.at());
            assert!(cursor.insert_after(input).is_ok());
            assert_eq!(input, *cursor.peek_next().unwrap());
            assert_eq!(Some(i), cursor.at());
            assert!(cursor.next().is_ok());
            assert_eq!(Some(i + 1), cursor.at());
        }
        assert_eq!(Some(inputs.len() - 1), cursor.at());

        // Remove all from tail
        assert!(cursor.seek_tail().is_ok());
        for (i, input) in inputs.into_iter().rev().enumerate() {
            assert_eq!(Some(inputs.len() - 1 - i), cursor.at());
            assert_eq!(Ok(input), cursor.remove());
        }
        assert_eq!(None, cursor.at());

        assert!(cursor.list_is_empty());
        assert!(!cursor.on_item());
        assert_eq!(0, cursor.list_len());

        // Re-populate the List
        cursor.insert_head(inputs[0]);
        assert_eq!(None, cursor.at());
        assert!(cursor.seek_head().is_ok());
        assert_eq!(Some(0), cursor.at());
        for (i, input) in inputs.into_iter().skip(1).enumerate() {
            assert_eq!(Some(i), cursor.at());
            assert!(cursor.insert_after(input).is_ok());
            assert_eq!(input, *cursor.peek_next().unwrap());
            assert_eq!(Some(i), cursor.at());
            assert!(cursor.next().is_ok());
            assert_eq!(Some(i + 1), cursor.at());
        }
        assert_eq!(Some(inputs.len() - 1), cursor.at());

        // Remove all from middle
        assert!(cursor.seek(4).is_ok());
        for input_idx in [4, 5, 6, 7, 8, 3, 2, 1, 0] {
            assert_eq!(Ok(inputs[input_idx]), cursor.remove());
        }

        assert!(cursor.list_is_empty());
        assert!(!cursor.on_item());
        assert_eq!(0, cursor.list_len());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_replace_a_value() {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!".to_owned(),
            "This".to_owned(),
            "is".to_owned(),
            "a".to_owned(),
            "test".to_owned(),
            "of".to_owned(),
            "the".to_owned(),
            "LinkedList".to_owned(),
            "cursor.".to_owned(),
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        cursor.insert_head(inputs[0].clone());
        assert!(cursor.seek_head().is_ok());
        for input in inputs.iter().skip(1) {
            assert!(cursor.insert_after(input.clone()).is_ok());
            assert!(cursor.next().is_ok());
        }

        // Traverse the list and modify the values
        assert!(cursor.seek_head().is_ok());
        for (i, input) in inputs.iter().enumerate() {
            assert!(cursor.replace(input.clone() + "XXX").is_ok());
            assert!(cursor.next().is_ok() || i == cursor.list_len() - 1);
        }

        // Check that the values were changed
        assert!(cursor.seek_head().is_ok());
        for (i, input) in inputs.iter().enumerate() {
            assert_eq!(input.clone() + "XXX", *cursor.peek().unwrap());
            assert!(cursor.next().is_ok() || i == cursor.list_len() - 1);
        }

        assert!(!cursor.list_is_empty());
        assert!(!cursor.on_item());
        assert_eq!(9, cursor.list_len());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_peek_mutably_at_the_current_value_and_both_inspect_and_mutate_it(
    ) {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!".to_owned(),
            "This".to_owned(),
            "is".to_owned(),
            "a".to_owned(),
            "test".to_owned(),
            "of".to_owned(),
            "the".to_owned(),
            "LinkedList".to_owned(),
            "cursor.".to_owned(),
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        cursor.insert_head(inputs[0].clone());
        assert!(cursor.seek_head().is_ok());
        for input in inputs.iter().skip(1) {
            assert!(cursor.insert_after(input.clone()).is_ok());
            assert!(cursor.next().is_ok());
        }

        // Traverse the list and modify the values
        assert!(cursor.seek_head().is_ok());
        for (i, input) in inputs.iter().enumerate() {
            *cursor.peek_mut().unwrap() = input.clone() + "XXX";
            assert!(cursor.next().is_ok() || i == cursor.list_len() - 1);
        }

        // Check that the values were changed
        assert!(cursor.seek_head().is_ok());
        for (i, input) in inputs.iter().enumerate() {
            assert_eq!(input.clone() + "XXX", *cursor.peek().unwrap());
            assert!(cursor.next().is_ok() || i == cursor.list_len() - 1);
        }

        assert!(!cursor.list_is_empty());
        assert!(!cursor.on_item());
        assert_eq!(9, cursor.list_len());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_peek_mutably_at_the_prev_value_and_both_inspect_and_mutate_it(
    ) {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!".to_owned(),
            "This".to_owned(),
            "is".to_owned(),
            "a".to_owned(),
            "test".to_owned(),
            "of".to_owned(),
            "the".to_owned(),
            "LinkedList".to_owned(),
            "cursor.".to_owned(),
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        cursor.insert_head(inputs[0].clone());
        assert!(cursor.seek_head().is_ok());
        for input in inputs.iter().skip(1) {
            assert!(cursor.insert_after(input.clone()).is_ok());
            assert!(cursor.next().is_ok());
        }

        // Traverse the list and modify the values
        assert!(cursor.seek(1).is_ok());
        #[allow(clippy::needless_range_loop)]
        for i in 0..inputs.len() - 1 {
            let suffix = if i & 1 == 0 { "XXX" } else { "" };
            *cursor.peek_prev_mut().unwrap() = inputs[i].clone() + suffix;
            assert!(cursor.seek_next(1).is_ok() || i == cursor.list_len() - 2);
        }

        // Check that the values were changed
        assert!(cursor.seek_head().is_ok());
        #[allow(clippy::needless_range_loop)]
        for i in 0..inputs.len() - 1 {
            let suffix = if i & 1 == 0 { "XXX" } else { "" };
            assert_eq!(inputs[i].clone() + suffix, *cursor.peek().unwrap());
            assert!(cursor.next().is_ok() || i == cursor.list_len() - 2);
        }

        assert!(!cursor.list_is_empty());
        assert!(cursor.on_item());
        assert_eq!(Some(8), cursor.at());
        assert_eq!(9, cursor.list_len());
    }

    #[test]
    fn it_allows_a_readwrite_cursor_to_peek_mutably_at_the_next_value_and_both_inspect_and_mutate_it(
    ) {
        let mut ll = LinkedList::new();

        let inputs = [
            "ATTENTION!!!".to_owned(),
            "This".to_owned(),
            "is".to_owned(),
            "a".to_owned(),
            "test".to_owned(),
            "of".to_owned(),
            "the".to_owned(),
            "LinkedList".to_owned(),
            "cursor.".to_owned(),
        ];

        let mut cursor = ll.cursor_mut(CursorStart::Head);
        cursor.insert_head(inputs[0].clone());
        assert!(cursor.seek_head().is_ok());
        for input in inputs.iter().skip(1) {
            assert!(cursor.insert_after(input.clone()).is_ok());
            assert!(cursor.next().is_ok());
        }

        // Traverse the list and modify the values
        assert!(cursor.seek(0).is_ok());
        #[allow(clippy::needless_range_loop)]
        for i in 0..inputs.len() - 1 {
            let suffix = if i & 1 == 1 { "XXX" } else { "" };
            *cursor.peek_next_mut().unwrap() = inputs[i + 1].clone() + suffix;
            assert!(cursor.seek_next(1).is_ok() || i == cursor.list_len() - 1);
        }

        // Check that the values were changed
        assert!(cursor.seek(1).is_ok());
        #[allow(clippy::needless_range_loop)]
        for i in 1..inputs.len() {
            let suffix = if i & 1 == 0 { "XXX" } else { "" };
            assert_eq!(inputs[i].clone() + suffix, *cursor.peek().unwrap());
            assert!(cursor.next().is_ok() || i == cursor.list_len() - 1);
        }

        assert!(!cursor.list_is_empty());
        assert!(!cursor.on_item());
        assert_eq!(None, cursor.at());
        assert_eq!(9, cursor.list_len());
    }
}
