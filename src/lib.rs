#[cfg(test)]
mod tests;

use std::{
    iter::Sum,
    ops::{Add, Sub},
};

use itertools::Itertools;

pub fn linear_search<T>(key: &T, items: &[T]) -> Option<usize>
where
    T: PartialEq,
{
    for (idx, item) in items.iter().enumerate() {
        if *key == *item {
            return Some(idx);
        }
    }
    None
}

pub fn binary_search<T>(key: &T, items: &[T]) -> Option<usize>
where
    T: PartialOrd,
{
    let mut start = 0;
    let mut end = items.len();
    while start < end {
        let check_idx = (start + end) / 2;
        if *key == items[check_idx] {
            return Some(check_idx);
        } else if *key < items[check_idx] {
            end = check_idx;
        } else {
            start = check_idx + 1;
        }
    }
    None
}

#[allow(clippy::manual_swap)]
pub fn reverse<T>(items: &mut [T])
where
    T: Copy,
{
    let mut start = 0;
    let mut end = items.len() - 1;
    while start < end {
        let tmp = items[start];
        items[start] = items[end];
        items[end] = tmp;
        start += 1;
        end -= 1;
    }
}

pub fn pairs<T>(items: &[T]) -> Vec<(T, T)>
where
    T: Copy,
{
    let mut pairs = Vec::new();
    for (i, item1) in items.iter().enumerate() {
        for item2 in items.iter().skip(i + 1) {
            pairs.push((*item1, *item2));
        }
    }
    pairs
}

pub fn pair_indices<T>(items: &[T]) -> Vec<(usize, usize)>
where
    T: Copy,
{
    let mut pairs = Vec::new();
    for (i, _) in items.iter().enumerate() {
        for (j, _) in items.iter().enumerate().skip(i + 1) {
            pairs.push((i, j));
        }
    }
    pairs
}

pub fn subarrays<T>(items: &[T]) -> Vec<&[T]> {
    let mut subarrays = Vec::new();
    for i in 0..items.len() {
        for j in i..items.len() {
            subarrays.push(&items[i..=j])
        }
    }
    subarrays
}

pub fn subarrays_sums<T>(items: &[T]) -> Vec<T>
where
    T: Copy + Sum,
{
    subarrays(items)
        .iter()
        .map(|a| a.iter().copied().sum())
        .collect()
}

pub fn subarrays_max_sum<T>(items: &[T]) -> Option<T>
where
    T: Copy + Sum + Ord,
{
    subarrays_sums(items).into_iter().max()
}

pub fn subarrays_sums_using_prefixes<T>(items: &[T]) -> Vec<T>
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T>,
{
    let mut prefix_sums = Vec::with_capacity(items.len());
    prefix_sums.push(items[0]);
    for (idx, item) in items.iter().enumerate().skip(1) {
        prefix_sums.push(prefix_sums[idx - 1] + *item);
    }
    let prefix_sums = prefix_sums;
    let mut subarray_sums = Vec::new();
    for i in 0..items.len() {
        for j in i..items.len() {
            subarray_sums.push(if i == 0 {
                prefix_sums[j]
            } else {
                prefix_sums[j] - prefix_sums[i - 1]
            })
        }
    }
    subarray_sums
}

pub fn subarrays_max_sum_using_prefixes<T>(items: &[T]) -> Option<T>
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Ord,
{
    subarrays_sums_using_prefixes(items).into_iter().max()
}

pub fn subarrays_max_sum_using_kadanes_algorithm<T>(items: &[T]) -> Option<T>
where
    T: Copy + Default + Add<T, Output = T> + Ord,
{
    let mut current_sum = T::default();
    let mut max_sum = T::default();
    for item in items {
        current_sum = (*item + current_sum).max(T::default());
        max_sum = max_sum.max(current_sum);
    }
    Some(max_sum)
}

pub fn sorted_pairs_sum<T>(items: &[T], closest_to: T) -> Option<(T, T)>
where
    T: Copy + Default + Ord + Add<T, Output = T> + Sub<T, Output = T>,
{
    pairs(items)
        .into_iter()
        .map(|(a, b)| (a + b, (a, b)))
        .map(|(sum, pair)| {
            (
                if sum <= closest_to {
                    closest_to - sum
                } else {
                    sum - closest_to
                },
                pair,
            )
        })
        .max_by_key(|(delta, _)| *delta)
        .map(|(_, pair)| pair)
}

#[allow(clippy::needless_collect)]
pub fn rotate<T>(items: &mut [T], times: usize)
where
    T: Copy,
{
    let len = items.len();
    if len <= 1 {
        return;
    }
    let times = times % len;
    if times == 0 {
        return;
    }
    let tmp = items.iter().take(times).copied().collect::<Vec<_>>();
    for i in times..len {
        items[i - times] = items[i];
    }
    for (i, item) in tmp.into_iter().enumerate() {
        items[len - times + i] = item;
    }
}

pub fn bubble_sort<T>(items: &mut [T])
where
    T: Copy + Ord,
{
    if items.len() <= 1 {
        return;
    }
    for i in 0..items.len() - 1 {
        let mut swapped = false;
        for j in 0..items.len() - 1 - i {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn insertion_sort<T>(items: &mut [T])
where
    T: Copy + Ord,
{
    if items.len() <= 1 {
        return;
    }
    for i in 1..items.len() {
        let tmp = items[i];
        for j in (0..i).rev() {
            if items[j] > tmp {
                items[j + 1] = items[j];
                items[j] = tmp;
            } else {
                break;
            }
        }
    }
}

pub fn selection_sort<T>(items: &mut [T])
where
    T: Copy + Ord,
{
    if items.len() <= 1 {
        return;
    }
    for i in 0..items.len() - 1 {
        let mut min_idx = i;
        for j in (i + 1)..items.len() {
            if items[min_idx] > items[j] {
                min_idx = j;
            }
        }
        items.swap(min_idx, i);
    }
}

pub fn counting_sort<T>(items: &mut [T])
where
    T: Copy + Ord + Add<T, Output = T> + Sub<T, Output = T> + Into<usize> + From<usize>,
{
    if items.len() <= 1 {
        return;
    }
    let (min, max) = match items.iter().minmax() {
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
        _ => unreachable!(),
    };
    let mut counts = vec![0_usize; (max - min).into() + 1];
    for item in items.iter() {
        counts[(*item - min).into()] += 1;
    }
    let mut item_idx = 0;
    for (count_idx, count) in counts.into_iter().enumerate() {
        for _ in 0..count {
            items[item_idx] = T::from(count_idx) + min;
            item_idx += 1;
        }
    }
}

pub fn chopsticks(items: &mut [u32], delta: u32) -> u32 {
    items.sort_unstable();
    let mut max_pairs = 0;
    let mut i = 0;
    while i < items.len() - 1 {
        if items[i].abs_diff(items[i + 1]) <= delta {
            i += 1;
            max_pairs += 1;
        }
        i += 1;
    }
    max_pairs
}

pub mod kingdom_defense {
    use std::collections::HashSet;

    pub fn kingdom_defense(width: usize, height: usize, positions: &mut [(usize, usize)]) -> u64 {
        max_undefended_territory_size(
            width,
            defended_columns(positions),
            height,
            defended_rows(positions),
        )
    }

    fn defended_columns(positions: &mut [(usize, usize)]) -> HashSet<usize> {
        positions
            .iter()
            .map(|(col, _)| *col)
            .collect::<HashSet<_>>()
    }

    fn defended_rows(positions: &mut [(usize, usize)]) -> HashSet<usize> {
        positions
            .iter()
            .map(|(_, row)| *row)
            .collect::<HashSet<_>>()
    }

    fn max_undefended_territory_size(
        width: usize,
        defended_cols: HashSet<usize>,
        height: usize,
        defended_rows: HashSet<usize>,
    ) -> u64 {
        let mut max_undefended_territory_size = 0;
        let mut col = 0;
        while col < width {
            let mut row = 0;
            if !defended_cols.contains(&col) {
                while row < width {
                    if !defended_rows.contains(&row)
                        && is_upper_left(col, &defended_cols, row, &defended_rows)
                    {
                        max_undefended_territory_size =
                            max_undefended_territory_size.max(undefended_territory_size(
                                col,
                                width,
                                &defended_cols,
                                row,
                                height,
                                &defended_rows,
                            ));
                    }
                    row += 1;
                }
            }
            col += 1;
        }
        max_undefended_territory_size as u64
    }

    #[allow(clippy::nonminimal_bool)]
    fn is_upper_left(
        col: usize,
        defended_cols: &HashSet<usize>,
        row: usize,
        defended_rows: &HashSet<usize>,
    ) -> bool {
        (col == 0 && row == 0)
            || (col == 0 && defended_rows.contains(&(row - 1)))
            || (row == 0 && defended_cols.contains(&(col - 1)))
            || ((col > 0 && defended_cols.contains(&(col - 1)))
                && (row > 0 && defended_rows.contains(&(row - 1))))
    }

    fn undefended_territory_size(
        col: usize,
        width: usize,
        defended_cols: &HashSet<usize>,
        row: usize,
        height: usize,
        defended_rows: &HashSet<usize>,
    ) -> usize {
        let mut rightmost_col = col;
        while rightmost_col < width - 1 && !defended_cols.contains(&(rightmost_col + 1)) {
            rightmost_col += 1;
        }
        let mut bottommost_row = row;
        while bottommost_row < height - 1 && !defended_rows.contains(&(bottommost_row + 1)) {
            bottommost_row += 1;
        }
        (rightmost_col - col + 1) * (bottommost_row - row + 1)
    }
}

pub mod shortest_path {

    pub fn shortest_path(moves: &str) -> String {
        let mut north_south = 0_i32;
        let mut east_west = 0_i32;
        for mv in moves.chars() {
            match mv {
                'N' | 'n' => north_south += 1,
                'S' | 's' => north_south -= 1,
                'E' | 'e' => east_west += 1,
                'W' | 'w' => east_west -= 1,
                _ => unreachable!("Input invalid"),
            }
        }
        let shortest_path = match north_south.cmp(&0) {
            std::cmp::Ordering::Greater => "N".repeat(north_south as usize),
            std::cmp::Ordering::Less => "S".repeat((-north_south) as usize),
            std::cmp::Ordering::Equal => "".into(),
        } + match east_west.cmp(&0) {
            std::cmp::Ordering::Greater => "E".repeat(east_west as usize),
            std::cmp::Ordering::Less => "W".repeat((-east_west) as usize),
            std::cmp::Ordering::Equal => "".into(),
        }
        .as_str();
        shortest_path
    }
}

pub mod palindrome {

    pub fn palindrome(phrase: &str) -> bool {
        phrase
            .chars()
            .zip(phrase.chars().rev())
            .all(|(a, b)| a == b)
    }
}

pub mod permutation {
    use std::collections::HashMap;

    pub fn permutation(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        if s1 == s2 {
            return true;
        }
        let mut char_counts_1 = HashMap::new();
        let mut char_counts_2 = HashMap::new();
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            char_counts_1
                .entry(c1)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            char_counts_2
                .entry(c2)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        char_counts_1 == char_counts_2
    }
}

pub mod remove_duplicates {
    use itertools::Itertools;

    pub fn remove_duplicates(s: &str) -> String {
        s.chars().sorted_unstable().dedup().collect()
    }
}

pub mod vowel_find {
    pub fn vowel_find(s: &str) -> String {
        s.chars().filter(is_vowel).collect()
    }

    fn is_vowel(c: &char) -> bool {
        "aeiouAEIOU".contains(*c)
    }
}

pub mod binary_string {
    pub fn binary_string(s: &str) -> u32 {
        let mut rv = 0;
        for c in s.chars() {
            match c {
                '0' => rv <<= 1,
                '1' => rv = (rv << 1) | 1,
                _ => panic!("Invalid Input"),
            }
        }
        rv
    }
}

pub mod print_2d {
    use std::ops::{Index, IndexMut};

    #[derive(Debug, Clone)]
    pub struct Vec2d<T>
    where
        T: Copy + Eq,
    {
        storage: Vec<T>,
        width: usize,
    }

    impl<T: Copy + Eq> Vec2d<T> {
        #[allow(dead_code)]
        pub fn new(width: usize, height: usize, initialize_with: T) -> Self {
            Self {
                storage: vec![initialize_with; width * height],
                width,
            }
        }
        pub fn width(&self) -> usize {
            self.width
        }
        pub fn height(&self) -> usize {
            if self.width == 0 {
                self.storage.len()
            } else {
                self.storage.len() / self.width
            }
        }
        pub fn size(&self) -> usize {
            self.storage.len()
        }
    }

    impl<T: Copy + Eq> From<Vec<Vec<T>>> for Vec2d<T> {
        fn from(values: Vec<Vec<T>>) -> Self {
            let width = if values.is_empty() {
                0
            } else {
                values[0].len()
            };
            Self {
                storage: values.into_iter().flat_map(|v| v.into_iter()).collect(),
                width,
            }
        }
    }

    impl<T: Copy + Eq> Index<(usize, usize)> for Vec2d<T> {
        type Output = T;

        fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
            assert!(col < self.width, "Column Index out-of-range");
            assert!(
                row < self.storage.len() / self.width,
                "Row Index out-of-range"
            );
            self.storage.index(row * self.width + col)
        }
    }

    impl<T: Copy + Eq> IndexMut<(usize, usize)> for Vec2d<T> {
        fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
            assert!(col < self.width, "Column Index out-of-range");
            assert!(
                row < self.storage.len() / self.width,
                "Row Index out-of-range"
            );
            self.storage.index_mut(row * self.width + col)
        }
    }

    impl<T: Copy + Eq> PartialEq for Vec2d<T> {
        fn eq(&self, other: &Self) -> bool {
            self.storage == other.storage && self.width == other.width
        }
    }

    impl<T: Copy + Eq> Eq for Vec2d<T> {}

    pub fn spiral_print(input: Vec2d<u32>) -> Vec<u32> {
        let mut col = 0;
        let mut row = 0;
        let mut direction = Direction::East;
        let mut top_row = 0;
        let mut left_column = 0;
        let mut bottom_row = input.height() - 1;
        let mut right_column = input.width() - 1;
        let mut rv = Vec::new();

        for _ in 0..(input.width() * input.height()) {
            rv.push(input[(col, row)]);
            match direction {
                Direction::East => {
                    if col < right_column {
                        col += 1;
                    } else {
                        direction = Direction::South;
                        row += 1;
                        top_row += 1;
                    };
                }
                Direction::South => {
                    if row < bottom_row {
                        row += 1
                    } else {
                        direction = Direction::West;
                        col -= 1;
                        right_column -= 1;
                    };
                }
                Direction::West => {
                    if col > left_column {
                        col -= 1
                    } else {
                        direction = Direction::North;
                        row -= 1;
                        bottom_row -= 1;
                    };
                }
                Direction::North => {
                    if row > top_row {
                        row -= 1;
                    } else {
                        direction = Direction::East;
                        col += 1;
                        left_column += 1;
                    }
                }
            }
        }
        rv
    }

    pub fn reverse_wave_print(input: Vec2d<u32>) -> Vec<u32> {
        let mut rv = Vec::new();
        for col in (0..input.width()).rev().step_by(2) {
            for row in 0..input.height() {
                rv.push(input[(col, row)]);
            }
            if col > 0 {
                for row in (0..input.height()).rev() {
                    rv.push(input[(col - 1, row)]);
                }
            }
        }
        rv
    }

    enum Direction {
        North,
        South,
        East,
        West,
    }
}

pub mod search_2d {
    use std::cmp::Ordering;

    use super::print_2d::Vec2d;

    pub fn staircase_search(input: Vec2d<u32>, find: u32) -> Option<(usize, usize)> {
        let (mut col, mut row) = (input.width() - 1, 0);
        loop {
            match find.cmp(&input[(col, row)]) {
                Ordering::Equal => return Some((col, row)),
                Ordering::Less => {
                    if col == 0 {
                        break;
                    }
                    col -= 1;
                }
                Ordering::Greater => {
                    if row == input.height() - 1 {
                        break;
                    }
                    row += 1;
                }
            }
        }
        None
    }
}

pub mod mango {
    use super::print_2d::Vec2d;

    pub fn best_split(input: Vec2d<u32>) -> u64 {
        let mut aux = Vec2d::new(input.width(), input.height(), 0_u64);
        for col in 0..input.width() {
            for row in 0..input.height() {
                aux[(col, row)] =                                                   //-
                    input[(col, row)] as u64                                        //-
                        + (if row==0 {0} else {aux[(col, row - 1)]})                //-
                        + (if col==0 {0} else {aux[(col - 1, row)]})                //-
                        - (if col==0 || row==0 {0} else {aux[(col - 1, row - 1)]});
            }
        }
        let mut max_reward = 0;
        for col in 0..input.width() {
            for row in 0..input.height() {
                let q1 = aux[(col, row)];
                let q2 = aux[(aux.width() - 1, row)] - q1;
                let q3 = aux[(col, aux.height() - 1)] - q1;
                let q4 = aux[(aux.width() - 1, aux.height() - 1)] - q1 - q2 - q3;
                max_reward = max_reward.max(q1.min(q2).min(q3).min(q4));
            }
        }
        max_reward
    }
}

pub mod pascal {
    pub fn pascals_triangle(depth: usize) -> Vec<Vec<u64>> {
        let mut rv = Vec::<Vec<u64>>::new();
        for row in 0..depth {
            let mut new_row = vec![1];
            if row > 0 {
                for col in 1..row {
                    let prev_row_column_before_value = rv[row - 1][col - 1];
                    let prev_row_column_value = if col >= row { 0 } else { rv[row - 1][col] };
                    new_row.push(prev_row_column_value + prev_row_column_before_value);
                }
                new_row.push(1);
            }
            rv.push(new_row);
        }
        rv
    }
}

pub mod matrix {
    use std::{
        cell::RefCell,
        ops::{Index, IndexMut},
    };

    pub struct Matrix2d<T>
    where
        T: Copy,
    {
        storage: Vec<T>,
        width: usize,
    }

    #[allow(dead_code)]
    impl<T: Copy> Matrix2d<T> {
        pub fn new(width: usize, height: usize, initialize_with: T) -> Self {
            Self {
                storage: vec![initialize_with; width * height],
                width,
            }
        }
        pub fn width(&self) -> usize {
            self.width
        }
        pub fn height(&self) -> usize {
            self.storage.len() / self.width
        }
        pub fn size(&self) -> usize {
            self.storage.len()
        }
        pub fn reduce_prefix_submatrices<U, InitU, CombinerUt, CombinerUu, RemoverUu>(
            &self,
            initializer_u: InitU,
            combiner_ut: CombinerUt,
            combiner_uu: CombinerUu,
            remover_uu: RemoverUu,
        ) -> Matrix2d<U>
        where
            U: Copy + Default,
            InitU: Fn() -> U,
            CombinerUt: Fn(&mut U, T),
            CombinerUu: Fn(&mut U, U),
            RemoverUu: Fn(&mut U, U),
        {
            let mut aux = Matrix2d::new(self.width(), self.height(), U::default());
            for col in 0..self.width() {
                for row in 0..self.height() {
                    let mut val = initializer_u();
                    combiner_ut(&mut val, self[(col, row)]);
                    if row > 0 {
                        combiner_uu(&mut val, aux[(col, row - 1)]);
                    }
                    if col > 0 {
                        combiner_uu(&mut val, aux[(col - 1, row)]);
                    }
                    if col > 0 && row > 0 {
                        remover_uu(&mut val, aux[(col - 1, row - 1)]);
                    }
                    aux[(col, row)] = val;
                }
            }
            aux
        }
    }

    impl<T: Copy> From<Vec<Vec<T>>> for Matrix2d<T> {
        fn from(values: Vec<Vec<T>>) -> Self {
            let width = values[0].len();
            let storage = values
                .into_iter()
                .flat_map(|v| v.into_iter())
                .collect::<Vec<T>>();
            assert!(storage.len() % width == 0);
            Self { storage, width }
        }
    }

    impl<T: Copy> From<(usize, Vec<T>)> for Matrix2d<T> {
        fn from((width, storage): (usize, Vec<T>)) -> Self {
            assert!(storage.len() % width == 0);
            Self { storage, width }
        }
    }

    impl<T: Copy> Index<(usize, usize)> for Matrix2d<T> {
        type Output = T;

        fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
            assert!(col < self.width, "Column Index out-of-range");
            assert!(
                row < self.storage.len() / self.width,
                "Row Index out-of-range"
            );
            self.storage.index(row * self.width + col)
        }
    }

    impl<T: Copy> IndexMut<(usize, usize)> for Matrix2d<T> {
        fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
            assert!(col < self.width, "Column Index out-of-range");
            assert!(
                row < self.storage.len() / self.width,
                "Row Index out-of-range"
            );
            self.storage.index_mut(row * self.width + col)
        }
    }

    pub struct QueryableMatrix2d<T, U, InitU, CombinerUt, CombinerUu, RemoverUu>
    where
        T: Copy,
        U: Copy + Default,
        InitU: Fn() -> U,
        CombinerUt: Fn(&mut U, T),
        CombinerUu: Fn(&mut U, U),
        RemoverUu: Fn(&mut U, U),
    {
        matrix: Matrix2d<T>,
        reduced_submatrices: RefCell<Option<Matrix2d<U>>>,
        initializer_u: InitU,
        combiner_ut: CombinerUt,
        combiner_uu: CombinerUu,
        remover_uu: RemoverUu,
    }

    #[allow(dead_code)]
    impl<T, U, InitU, CombinerUt, CombinerUu, RemoverUu>
        QueryableMatrix2d<T, U, InitU, CombinerUt, CombinerUu, RemoverUu>
    where
        T: Copy,
        U: Copy + Default,
        InitU: Fn() -> U,
        CombinerUt: Fn(&mut U, T),
        CombinerUu: Fn(&mut U, U),
        RemoverUu: Fn(&mut U, U),
    {
        pub fn reduced_submatrix_value(
            &self,
            col1: usize,
            row1: usize,
            col2: usize,
            row2: usize,
        ) -> U {
            self.reduced_submatrices
                .borrow_mut()
                .get_or_insert_with(|| {
                    self.matrix.reduce_prefix_submatrices(
                        &self.initializer_u,
                        &self.combiner_ut,
                        &self.combiner_uu,
                        &self.remover_uu,
                    )
                });
            let rv = self.reduced_submatrices.borrow().as_ref().map(|aux| {
                let mut rv = aux[(col2, row2)];
                if col1 > 0 && row1 > 0 {
                    (self.combiner_uu)(&mut rv, aux[(col1 - 1, row1 - 1)])
                }
                if row1 > 0 {
                    (self.remover_uu)(&mut rv, aux[(col2, row1 - 1)])
                };
                if col1 > 0 {
                    (self.remover_uu)(&mut rv, aux[(col1 - 1, row2)])
                };
                rv
            });
            rv.unwrap()
        }
    }

    impl<T, U, InitU, CombinerUt, CombinerUu, RemoverUu>
        From<(usize, Vec<T>, InitU, CombinerUt, CombinerUu, RemoverUu)>
        for QueryableMatrix2d<T, U, InitU, CombinerUt, CombinerUu, RemoverUu>
    where
        T: Copy,
        U: Copy + Default,
        InitU: Fn() -> U,
        CombinerUt: Fn(&mut U, T),
        CombinerUu: Fn(&mut U, U),
        RemoverUu: Fn(&mut U, U),
    {
        fn from(
            (width, storage, initializer_u, combiner_ut, combiner_uu, remover_uu): (
                usize,
                Vec<T>,
                InitU,
                CombinerUt,
                CombinerUu,
                RemoverUu,
            ),
        ) -> Self {
            Self {
                matrix: Matrix2d::from((width, storage)),
                reduced_submatrices: RefCell::new(None),
                initializer_u,
                combiner_ut,
                combiner_uu,
                remover_uu,
            }
        }
    }
}

pub mod vector {
    use std::ops;

    // Safety: Invariants - self.len <= buf.capacity() at all times and self.buf contains self.len live elements
    pub struct Vector<T> {
        buf: raw::RawVector<T>,
        len: usize,
    }

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Self {
                buf: raw::RawVector::new(),
                len: 0,
            }
        }

        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                buf: raw::RawVector::with_capacity(capacity),
                len: 0,
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn capacity(&self) -> usize {
            self.buf.capacity()
        }

        pub fn push(&mut self, element: T) {
            self.buf.push(element);
            self.len += 1;
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            }
            let popped = self.buf.pop();
            self.len -= 1;
            Some(popped)
        }
    }

    impl<T> Default for Vector<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> ops::Index<usize> for Vector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            assert!(
                index < self.len,
                "Index out-of-range: Length {len}, Index {index}",
                len = self.len
            );
            self.buf.index(index)
        }
    }

    impl<T> ops::IndexMut<usize> for Vector<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            assert!(
                index < self.len,
                "Index out-of-range: Length {len}, Index {index}",
                len = self.len
            );
            self.buf.index_mut(index)
        }
    }

    mod raw {
        use std::{alloc, ops, ptr};

        // Safety:
        //   - Invariant - capacity is zero, then buf is null;
        //     otherwise, capacity is non-zero, then buf points to a memory of the necessary size
        //     and alignment for the type and capacity
        //   - self.len <= self.capacity at all times
        pub(super) struct RawVector<T> {
            capacity: usize,
            len: usize,
            buf: *const T,
        }

        impl<T> RawVector<T> {
            pub fn new() -> Self {
                Self {
                    capacity: 0,
                    len: 0,
                    buf: ptr::null(),
                }
            }

            pub fn with_capacity(capacity: usize) -> Self {
                if capacity == 0 {
                    return Self::new();
                }
                let buf = Self::allocate_new_buffer(capacity);
                // Safety: The capacity and the allocation size will always match here
                Self {
                    capacity,
                    len: 0,
                    buf,
                }
            }

            pub fn capacity(&self) -> usize {
                self.capacity
            }

            fn debug_assert_safety_invariants_maintained(&self) {
                // Safety: ensure something erroneous didn't happen and that the type invariants were properly maintained
                debug_assert!(
                    (self.capacity == 0 && self.buf.is_null() && self.len == 0)
                        || (self.capacity > 0 && !self.buf.is_null() && self.len <= self.capacity),
                    "SAFETY: Vector invariants violated: capacity {capacity}, length {len}, buf {buf}",
                    capacity = self.capacity,
                    len = self.len,
                    buf = if self.buf.is_null() {
                        "null"
                    } else {
                        "not null"
                    }
                );
            }

            fn layout(capacity: usize) -> alloc::Layout {
                alloc::Layout::array::<T>(capacity).expect("layout construction succeeded")
            }

            pub fn push(&mut self, element: T) {
                self.debug_assert_safety_invariants_maintained();
                let at = self.len.try_into().expect("self.len < isize::MAX");
                self.ensure_capacity(at);
                // Safety: storing element to 1 past the last stored element due to safety contract
                //         of this function and we've ensured that sufficient capacity is available
                //   - verified 'at' < self.len which is invariant < self.capacity
                let buf = self.buf as *mut T;
                unsafe { buf.offset(at).write(element) }
                self.len += 1;
            }

            pub fn pop(&mut self) -> T {
                self.debug_assert_safety_invariants_maintained();
                let from: isize = self.len.try_into().expect("self.len <= isize::MAX");
                debug_assert!(from >= 0, "RawVector::pop - self.len = 0, nothing to pop");
                debug_assert!(
                    self.len < self.capacity,
                    "RawVector::pop - self.len must be <= self.capacity it is {from} and capacity is {capacity}",
                    capacity = self.capacity
                );
                // Safety:
                //   - ptr returned by offset(from) will be in-bounds on self.buf due to verifying that from < self.capacity
                //   - from will point to last "live" element by the contract of this function
                //   - alignment will be correct due to proper usage of alloc/layout
                //   - self.capacity is decremented so that the last "live" element is now the previous element
                let rv = unsafe { (self.buf as *mut T).offset(from - 1).read() };
                self.len -= 1;
                rv
            }

            fn ensure_capacity(&mut self, index: isize) {
                let index = index.try_into().expect("index is >= 0");
                if self.capacity > index {
                    return;
                }
                let mut new_capacity = self.capacity.max(16);
                while new_capacity <= index {
                    new_capacity *= 2
                }
                let new_buf = Self::allocate_new_buffer(new_capacity) as *mut T;
                // Safety:
                //   - self.buf is guaranteed self.capacity in length;
                //   - new_buf is new_capacity in length which is guaranteed at least 2x self.capacity;
                //   - self.buf and new_buf were properly created using alloc/layout;
                //   - as new_buf is a new allocation it is guaranteed to not overlap with self.buf
                unsafe { self.buf.copy_to_nonoverlapping(new_buf, self.capacity) }
                let old_buf = self.buf as *mut u8;
                // Safety:
                //   - old_buf uses the layout corresponding to the existing self.capacity
                //   - the memory of old_buf has been copied to new_buf
                //   - no other ptr's alias to self.buf
                unsafe { alloc::dealloc(old_buf, Self::layout(self.capacity)) };
                self.buf = new_buf;
                self.capacity = new_capacity;
            }

            fn allocate_new_buffer(capacity: usize) -> *const T {
                let layout = Self::layout(capacity);
                if layout.size() == 0 {
                    alloc::handle_alloc_error(layout);
                }
                let buf = unsafe { alloc::alloc(layout) as *const T };
                if buf.is_null() {
                    alloc::handle_alloc_error(layout);
                }
                buf
            }
        }

        impl<T> ops::Index<usize> for RawVector<T> {
            type Output = T;
            fn index(&self, index: usize) -> &Self::Output {
                self.debug_assert_safety_invariants_maintained();
                debug_assert!(
                    index < self.len,
                    "Index out-of-range: Length {len}, Index {index}",
                    len = self.len
                );
                let index = index.try_into().expect("index is < isize::MAX");
                // Safety:
                //   - index guaranteed in range as self.len is guaranteed by type invariants to be less than self.capacity
                //   - self.buf is not null if self.len > 0 as guaranteed by type invariants
                unsafe {
                    self.buf
                        .offset(index)
                        .as_ref()
                        .expect("self.buf is not null")
                }
            }
        }

        impl<T> ops::IndexMut<usize> for RawVector<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                self.debug_assert_safety_invariants_maintained();
                debug_assert!(
                    index < self.len,
                    "Index out-of-range: Length {len}, Index {index}",
                    len = self.len
                );
                let index = index.try_into().expect("index is < isize::MAX");
                // Safety:
                //   - index guaranteed in range as self.len is guaranteed by type invariants to be less than self.capacity
                //   - self.buf is not null if self.len > 0 as guaranteed by type invariants
                unsafe {
                    (self.buf as *mut T)
                        .offset(index)
                        .as_mut()
                        .expect("self.buf is not null")
                }
            }
        }

        impl<T> Drop for RawVector<T> {
            fn drop(&mut self) {
                self.debug_assert_safety_invariants_maintained();
                if !self.buf.is_null() {
                    // Safety: ensured same Layout is used and that buf is not null and invariants were maintained
                    //    - all items are being properly dropped
                    let layout = Self::layout(self.capacity);
                    unsafe {
                        for i in 0..self.len.try_into().expect("self.len <= isize::MAX") {
                            (self.buf.offset(i) as *mut T).drop_in_place();
                        }
                        alloc::dealloc(self.buf as *mut u8, layout);
                    }
                }
            }
        }
    }
}

pub mod cabs {
    use itertools::Itertools;

    pub fn sort_by_distance_from_origin(
        cab_locations: impl IntoIterator<Item = (i32, i32)>,
    ) -> Vec<(i32, i32)> {
        cab_locations
            .into_iter()
            .sorted_unstable_by_key(distance_from_origin)
            .collect()
    }

    fn distance_from_origin(coord: &(i32, i32)) -> i64 {
        ((coord.0 as f32).powi(2) + (coord.1 as f32).powi(2))
            .sqrt()
            .round() as i64
    }
}

pub mod fruits {
    use itertools::Itertools;

    pub fn sort_by(
        fruits: impl IntoIterator<Item = (String, u32)>,
        key: FruitSortKey,
    ) -> Vec<(String, u32)> {
        match key {
            FruitSortKey::Name => fruits
                .into_iter()
                .sorted_unstable_by_key(fruit_name)
                .collect(),
            FruitSortKey::Cost => fruits
                .into_iter()
                .sorted_unstable_by_key(fruit_cost)
                .collect(),
        }
    }

    fn fruit_name(fruit: &(String, u32)) -> String {
        fruit.0.clone()
    }

    fn fruit_cost(fruit: &(String, u32)) -> u32 {
        fruit.1
    }

    pub enum FruitSortKey {
        Name,
        Cost,
    }
}

pub mod image {
    use super::print_2d::Vec2d;

    pub fn rotate_90_degrees(image: &mut Vec2d<u32>) {
        assert!(image.width() == image.height(), "image must be square");
        if image.width() <= 1 {
            return;
        }

        let mut top_idx = (0, 0);
        let mut left_idx = (0, image.height() - 1);
        let mut bottom_idx = (image.width() - 1, image.height() - 1);
        let mut right_idx = (image.width() - 1, 0);

        for round in 0..(image.width() / 2) {
            for i in 0..(image.width() - 1 - 2 * round) {
                let tmp_top = image[(top_idx.0 + i, top_idx.1)];
                image[(top_idx.0 + i, top_idx.1)] = image[(left_idx.0, left_idx.1 - i)];
                image[(left_idx.0, left_idx.1 - i)] = image[(bottom_idx.0 - i, bottom_idx.1)];
                image[(bottom_idx.0 - i, bottom_idx.1)] = image[(right_idx.0, right_idx.1 + i)];
                image[(right_idx.0, right_idx.1 + i)] = tmp_top;
            }
            top_idx.0 += 1;
            top_idx.1 += 1;
            left_idx.0 += 1;
            left_idx.1 -= 1;
            bottom_idx.0 -= 1;
            bottom_idx.1 -= 1;
            right_idx.0 -= 1;
            right_idx.1 += 1;
        }
    }
}

pub mod zeroes {
    use std::collections::HashSet;

    use super::print_2d::Vec2d;

    pub fn make_zeroes(mut matrix: Vec2d<u32>) -> Vec2d<u32> {
        let mut columns_to_zero = HashSet::new();
        let mut rows_to_zero = HashSet::new();
        for col in 0..matrix.width() {
            for row in 0..matrix.height() {
                if matrix[(col, row)] == 0 {
                    columns_to_zero.insert(col);
                    rows_to_zero.insert(row);
                }
            }
        }
        for col in columns_to_zero {
            for row in 0..matrix.height() {
                matrix[(col, row)] = 0;
            }
        }
        for row in rows_to_zero {
            for col in 0..matrix.width() {
                matrix[(col, row)] = 0;
            }
        }
        matrix
    }
}

pub mod xoring {

    // LIMITATIONS: This function will produce a meaningless result if there is more than 1 unique number in the list (including zero)
    pub fn find_unique_number(input: impl IntoIterator<Item = u32>) -> Option<u32> {
        let mut count_of_zeroes = 0;
        let rv = input
            .into_iter()
            .inspect(|v| {
                if *v == 0 {
                    count_of_zeroes += 1
                }
            })
            .reduce(|accum, next| accum ^ next);
        if let Some(0) = rv {
            if count_of_zeroes & 1 == 1 {
                return Some(0);
            } else {
                return None;
            }
        }
        rv
    }
}

pub mod modulus {

    pub fn powmod(base: u32, exp: u32, m: u32) -> u32 {
        let mut base = base as u64;
        let mut exp = exp;
        let m = m as u64;

        let mut result = 1 % m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % m
            }
            base = (base * base) % m;
            exp >>= 1;
        }
        result as u32
    }
}

pub mod earth {

    pub fn level_up_by_powers_of_2(mut level_to_reach: u32) -> u32 {
        let mut rv = 0;
        while level_to_reach > 0 {
            let mut first_jump = level_to_reach;
            while first_jump & (first_jump - 1) > 0 {
                first_jump -= 1;
            }
            rv += 1;
            level_to_reach -= first_jump;
        }
        rv
    }
}

pub mod subset {

    use std::cmp::Ordering;

    //use awint::prelude::*;
    use bitvec::prelude::*;

    pub fn subset_sum_queries(data: Vec<i32>, queries: impl IntoIterator<Item = u32>) -> Vec<bool> {
        let subset_sum_flags = compute_subset_sum_flags(data);
        compute_list_of_queries_results(subset_sum_flags, queries)
    }

    fn compute_list_of_queries_results(
        subset_sum_flags: BitVec,
        queries: impl IntoIterator<Item = u32>,
    ) -> Vec<bool> {
        queries
            .into_iter()
            .map(|sum| subset_sum_flags[sum as usize])
            .collect()
    }

    fn compute_subset_sum_flags(data: Vec<i32>) -> BitVec {
        let mut subset_sum_flags = bitvec![0_u64;100_000];
        subset_sum_flags.set(0, true);
        for val in data {
            let original_subset_sum_flags = subset_sum_flags.clone();
            match val.cmp(&0) {
                Ordering::Greater => {
                    subset_sum_flags.shift_right(val as usize); // NOTE: The API used here seemingly reads backwards - this is actually performing an LSL (logical shift left)
                }
                Ordering::Less => {
                    subset_sum_flags.shift_left(-val as usize);
                }
                Ordering::Equal => (),
            }
            subset_sum_flags |= original_subset_sum_flags;
        }
        subset_sum_flags
    }
}

pub mod recursion {
    use std::collections::HashMap;

    pub fn factorial(n: u8) -> u64 {
        if n == 0 {
            return 1;
        }
        factorial_tc(n, 1)
    }
    fn factorial_tc(n: u8, fact: u64) -> u64 {
        if n == 0 {
            return fact;
        }
        factorial_tc(n - 1, n as u64 * fact)
    }

    pub fn fibonacci(n: u16) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        fibonacci(n - 1) + fibonacci(n - 2)
    }
    pub fn fibonacci_tc(n: u16) -> u64 {
        fibonacci_tch(n, 0, 1)
    }
    fn fibonacci_tch(n: u16, a: u64, b: u64) -> u64 {
        if n == 0 {
            return a;
        }
        if n == 1 {
            return b;
        }
        fibonacci_tch(n - 1, b, a + b)
    }

    pub fn is_array_sorted<T: Ord>(arr: &[T]) -> bool {
        if arr.len() <= 1 {
            return true;
        }
        arr[0] <= arr[1] && is_array_sorted(&arr[1..])
    }
    pub fn is_array_sorted_tc<T: Ord>(arr: &[T]) -> bool {
        is_array_sorted_tch(true, arr)
    }
    pub fn is_array_sorted_tch<T: Ord>(prefix_sorted: bool, arr: &[T]) -> bool {
        if arr.len() <= 1 {
            return prefix_sorted;
        }
        is_array_sorted_tch(arr[0] <= arr[1] && prefix_sorted, &arr[1..])
    }

    pub fn find_all_occurences<T: Ord + Copy>(search_for: &T, data: &[T]) -> Vec<usize> {
        find_all_occurences_tc(search_for, data, 0, Vec::new())
    }
    fn find_all_occurences_tc<T: Ord + Copy>(
        search_for: &T,
        data: &[T],
        position: usize,
        mut found: Vec<usize>,
    ) -> Vec<usize> {
        if data.is_empty() {
            return found;
        }
        if data[0] == *search_for {
            found.push(position);
        }
        find_all_occurences_tc(search_for, &data[1..], position + 1, found)
    }

    pub fn find_number_of_possible_tilings(floor_size: (u32, u32), tile_size: (u32, u32)) -> u64 {
        if floor_size == tile_size || floor_size == rotated(tile_size) || area(floor_size) == 0 {
            return 1;
        }
        if area(floor_size) < area(tile_size) || area(tile_size) == 0 {
            return 0;
        }
        number_of_tilings_nonrotated(tile_size, floor_size)
            + number_of_tilings_rotated(tile_size, floor_size)
    }
    fn area(size: (u32, u32)) -> u64 {
        size.0 as u64 * size.1 as u64
    }
    fn rotated(tile_size: (u32, u32)) -> (u32, u32) {
        (tile_size.1, tile_size.0)
    }
    fn number_of_tilings_nonrotated(tile_size: (u32, u32), floor_size: (u32, u32)) -> u64 {
        if fits_within(tile_size, floor_size) {
            number_of_tilings(tile_size, floor_size)
        } else {
            0
        }
    }
    fn number_of_tilings_rotated(tile_size: (u32, u32), floor_size: (u32, u32)) -> u64 {
        if tile_size != rotated(tile_size) && fits_within(rotated(tile_size), floor_size) {
            number_of_tilings(rotated(tile_size), floor_size)
        } else {
            0
        }
    }
    fn fits_within(tile_size: (u32, u32), floor_size: (u32, u32)) -> bool {
        floor_size.0 >= tile_size.0 && floor_size.1 >= tile_size.1
    }
    fn number_of_tilings(tile_size: (u32, u32), floor_size: (u32, u32)) -> u64 {
        remaining_sets_of_pieces_after_fit(tile_size, floor_size)
            .into_iter()
            .map(|set_of_pieces| number_of_tilings_for_set_of_pieces(set_of_pieces, tile_size))
            .max()
            .unwrap_or(0)
    }
    fn number_of_tilings_for_set_of_pieces(pieces: Vec<(u32, u32)>, tile_size: (u32, u32)) -> u64 {
        pieces
            .into_iter()
            .map(|floor_piece_size| find_number_of_possible_tilings(floor_piece_size, tile_size))
            .product::<u64>()
    }
    fn remaining_sets_of_pieces_after_fit(
        tile_size: (u32, u32),
        floor_size: (u32, u32),
    ) -> Vec<Vec<(u32, u32)>> {
        // tile in upper-left
        let piece_lower_right = (floor_size.0 - tile_size.0, floor_size.1 - tile_size.1);
        let piece_bottom_left = (tile_size.0, floor_size.1 - tile_size.1);
        let piece_upper_right = (floor_size.0 - tile_size.0, tile_size.1);
        let piece_right = (floor_size.0 - tile_size.0, floor_size.1);
        let piece_bottom = (floor_size.0, floor_size.1 - tile_size.1);
        vec![
            vec![
                piece_lower_right, //
                piece_bottom_left, //
                piece_upper_right, //
            ],
            vec![
                piece_right,       //
                piece_bottom_left, //
            ],
            vec![
                piece_bottom,      //
                piece_upper_right, //
            ],
        ]
    }

    pub fn binary_strings_with_nonrepeating_ones_of_length_sorted(length: u8) -> Vec<String> {
        assert!(length <= 12, "Not supported for length > 12");
        binary_strings(length)
    }
    fn binary_strings(length: u8) -> Vec<String> {
        let mut rv = Vec::new();
        if length == 0 {
        } else if length == 1 {
            rv.push("0".into());
            rv.push("1".into());
        } else {
            let bss = binary_strings(length - 1);
            bss.iter()
                .map(|s| "0".to_owned() + s.as_str())
                .for_each(|s| rv.push(s));
            bss.into_iter()
                .filter(|s| s.starts_with('0'))
                .map(|s| "1".to_owned() + s.as_str())
                .for_each(|s| rv.push(s));
        }
        rv
    }

    pub fn party_pairings(n: u8) -> u64 {
        let mut saved_computations = HashMap::<u8, u64>::new();
        party_pairings_memoized(n, &mut saved_computations)
    }
    fn party_pairings_memoized(n: u8, saved_computations: &mut HashMap<u8, u64>) -> u64 {
        if n <= 1 {
            return 1;
        }
        *saved_computations
            .entry(n)
            .or_insert_with(|| party_pairings(n - 1) + (n as u64 - 1) * party_pairings(n - 2))
    }
}

pub mod divide_conquer {

    pub mod sort {
        use crate::print_2d::Vec2d;

        pub fn merge_sort<T: Ord + Copy>(data: &mut [T]) {
            if data.len() <= 1 {
                return;
            }
            let mid = data.len() / 2;
            let (left, right) = data.split_at_mut(mid);
            merge_sort(left);
            merge_sort(right);
            merge(left, right);
        }
        fn merge<T: Ord + Copy>(left: &mut [T], right: &mut [T]) {
            let mut l_idx = 0;
            let mut r_idx = 0;
            let mut tmp = Vec::with_capacity(left.len() + right.len());
            while l_idx < left.len() || r_idx < right.len() {
                if r_idx >= right.len() || (l_idx < left.len() && left[l_idx] <= right[r_idx]) {
                    tmp.push(left[l_idx]);
                    l_idx += 1;
                } else if l_idx >= left.len() || left[l_idx] > right[r_idx] {
                    tmp.push(right[r_idx]);
                    r_idx += 1;
                }
            }
            for (idx, val) in tmp.iter().take(left.len()).enumerate() {
                left[idx] = *val;
            }
            for (idx, val) in tmp.iter().skip(left.len()).enumerate() {
                right[idx] = *val;
            }
        }

        pub fn quick_sort<T: Ord + Copy>(data: &mut [T]) {
            if data.len() <= 1 {
                return;
            }
            let pivot = partition(data);
            let (left, right) = data.split_at_mut(pivot);
            quick_sort(left);
            quick_sort(right);
        }
        fn partition<T: Ord + Copy>(data: &mut [T]) -> usize {
            if data.len() <= 1 {
                return 0;
            }
            let mut pivot = data.len() - 1;
            let mut start_of_above = 0;
            let mut current_compare = 0;
            while current_compare < pivot {
                if data[current_compare] <= data[pivot] {
                    if current_compare > start_of_above {
                        data.swap(start_of_above, current_compare);
                    }
                    start_of_above += 1;
                }
                current_compare += 1;
            }
            if start_of_above < pivot {
                data.swap(start_of_above, pivot);
                pivot = start_of_above;
            }
            pivot
        }

        pub fn merge_sort_2d<T: Ord + Copy>(data: &mut Vec2d<T>) {
            let (col_s, col_e, row_s, row_e) = (0, data.width() - 1, 0, data.height() - 1);
            merge_sort_2d_rec(data, col_s, col_e, row_s, row_e);
        }
        fn merge_sort_2d_rec<T: Ord + Copy>(
            data: &mut Vec2d<T>,
            col_s: usize,
            col_e: usize,
            row_s: usize,
            row_e: usize,
        ) {
            // if zero-sized in row or column or sized exactly 1x1 - nothing to do
            if col_s > col_e || row_s > row_e || (col_s == col_e && row_s == row_e) {
                return;
            }
            // sort upper-left
            merge_sort_2d_rec(
                data, //
                col_s,
                (col_s + col_e) / 2,
                row_s,
                (row_s + row_e) / 2,
            );
            // sort upper-right
            merge_sort_2d_rec(
                data, //
                (col_s + col_e) / 2 + 1,
                col_e,
                row_s,
                (row_s + row_e) / 2,
            );
            // sort lower-left
            merge_sort_2d_rec(
                data, //
                col_s,
                (col_s + col_e) / 2,
                (row_s + row_e) / 2 + 1,
                row_e,
            );
            // sort lower-right
            merge_sort_2d_rec(
                data, //
                (col_s + col_e) / 2 + 1,
                col_e,
                (row_s + row_e) / 2 + 1,
                row_e,
            );
            // merge upper-left && upper-right
            merge_2d(
                data, //
                col_s,
                (col_s + col_e) / 2,
                row_s,
                (row_s + row_e) / 2,
                (col_s + col_e) / 2 + 1,
                col_e,
                row_s,
                (row_s + row_e) / 2,
            );
            // merge lower-left & lower-right
            merge_2d(
                data, //
                col_s,
                (col_s + col_e) / 2,
                (row_s + row_e) / 2 + 1,
                row_e,
                (col_s + col_e) / 2 + 1,
                col_e,
                (row_s + row_e) / 2 + 1,
                row_e,
            );
            // merge full top & bottom
            merge_2d(
                data, //
                col_s,
                col_e,
                row_s,
                (row_s + row_e) / 2,
                col_s,
                col_e,
                (row_s + row_e) / 2 + 1,
                row_e,
            )
        }
        #[allow(clippy::too_many_arguments)]
        fn merge_2d<T: Ord + Copy>(
            data: &mut Vec2d<T>,
            col_s1: usize,
            col_e1: usize,
            row_s1: usize,
            row_e1: usize,
            col_s2: usize,
            col_e2: usize,
            row_s2: usize,
            row_e2: usize,
        ) {
            if col_s1 < col_s2 && col_s1 <= col_e1 && col_s2 <= col_e2 {
                for r_idx in row_s1..=row_e2 {
                    merge_2d_row(data, col_s1, col_e1, col_s2, col_e2, r_idx);
                }
            }
            if row_s1 < row_s2 && row_s1 <= row_e1 && row_s2 <= row_e2 {
                for c_idx in col_s1..=col_e2 {
                    merge_2d_col(data, row_s1, row_e1, row_s2, row_e2, c_idx);
                }
            }
        }
        fn merge_2d_row<T: Ord + Copy>(
            data: &mut Vec2d<T>,
            col_s1: usize,
            col_e1: usize,
            col_s2: usize,
            col_e2: usize,
            row: usize,
        ) {
            let mut l_idx = 0;
            let mut r_idx = 0;
            let left_len = col_e1 - col_s1 + 1;
            let right_len = col_e2 - col_s2 + 1;
            let mut tmp = Vec::with_capacity(col_e2 - col_s1 + 1);
            while l_idx < left_len || r_idx < right_len {
                if r_idx >= right_len
                    || (l_idx < left_len
                        && data[(col_s1 + l_idx, row)] <= data[(col_s2 + r_idx, row)])
                {
                    tmp.push(data[(col_s1 + l_idx, row)]);
                    l_idx += 1;
                } else if l_idx >= left_len
                    || data[(col_s1 + l_idx, row)] > data[(col_s2 + r_idx, row)]
                {
                    tmp.push(data[(col_s2 + r_idx, row)]);
                    r_idx += 1;
                }
            }
            for (idx, val) in tmp.iter().enumerate() {
                data[(col_s1 + idx, row)] = *val;
            }
        }
        fn merge_2d_col<T: Ord + Copy>(
            data: &mut Vec2d<T>,
            row_s1: usize,
            row_e1: usize,
            row_s2: usize,
            row_e2: usize,
            col: usize,
        ) {
            let mut t_idx = 0;
            let mut b_idx = 0;
            let top_len = row_e1 - row_s1 + 1;
            let bottom_len = row_e2 - row_s2 + 1;
            let mut tmp = Vec::with_capacity(row_e2 - row_s1 + 1);
            while t_idx < top_len || b_idx < bottom_len {
                if b_idx >= bottom_len
                    || (t_idx < top_len
                        && data[(col, row_s1 + t_idx)] <= data[(col, row_s2 + b_idx)])
                {
                    tmp.push(data[(col, row_s1 + t_idx)]);
                    t_idx += 1;
                } else if t_idx >= top_len
                    || data[(col, row_s1 + t_idx)] > data[(col, row_s2 + b_idx)]
                {
                    tmp.push(data[(col, row_s2 + b_idx)]);
                    b_idx += 1;
                }
            }
            for (idx, val) in tmp.iter().enumerate() {
                data[(col, row_s1 + idx)] = *val;
            }
        }
    }

    pub mod search {
        use std::cmp::Ordering;

        pub fn binary_search<T: Ord>(find: &T, data: &[T]) -> Option<usize> {
            if data.is_empty() {
                return None;
            }
            let mid = data.len() / 2;
            match find.cmp(&data[mid]) {
                Ordering::Equal => Some(mid),
                Ordering::Less => binary_search(find, &data[0..mid]),
                Ordering::Greater => binary_search(find, &data[mid + 1..]).map(|m| mid + m + 1),
            }
        }

        pub fn rotated_binary_search<T: Ord>(find: &T, data: &[T]) -> Option<usize> {
            if data.is_empty() {
                return None;
            }
            let mut s = 0;
            let mut e = data.len() - 1;
            while s <= e {
                let mid = (e + s) / 2;
                match find.cmp(&data[mid]) {
                    Ordering::Equal => return Some(mid),
                    Ordering::Less if *find >= data[s] && mid > 0 => e = mid - 1,
                    Ordering::Less => s = mid + 1,
                    Ordering::Greater if *find <= data[e] => s = mid + 1,
                    Ordering::Greater if mid > 0 => e = mid - 1,
                    _ => return None,
                }
            }
            None
        }
    }
}

pub mod backtracking {

    pub mod permutations {
        pub fn permutations<T: Copy>(data: &mut [T]) -> Vec<Vec<T>> {
            let mut permutations = Vec::new();
            let mut current_permutation = Vec::new();
            permutations_rec(data, 0, &mut current_permutation, &mut permutations);
            permutations
        }
        fn permutations_rec<T: Copy>(
            data: &mut [T],
            s: usize,
            current_permutation: &mut Vec<T>,
            permutations: &mut Vec<Vec<T>>,
        ) {
            if s == data.len() {
                permutations.push(current_permutation.clone());
                return;
            }
            for i in s..data.len() {
                current_permutation.push(data[i]);
                if i != s {
                    data.swap(s, i);
                }
                permutations_rec(data, s + 1, current_permutation, permutations);
                if i != s {
                    data.swap(s, i);
                }
                current_permutation.pop();
            }
        }
    }

    pub mod n_queens {

        pub fn n_queen_find_a_solution(n: usize) -> Option<Vec<(usize, usize)>> {
            let mut solution = Vec::new();
            match n_queen_single_solution(n, 0, &mut solution) {
                true => Some(solution),
                false => None,
            }
        }
        fn n_queen_single_solution(
            n: usize,
            row: usize,
            solution: &mut Vec<(usize, usize)>,
        ) -> bool {
            if row == n {
                return true;
            }
            for col in 0..n {
                match n_queen_safe((col, row), solution) {
                    true => {
                        solution.push((col, row));
                        match n_queen_single_solution(n, row + 1, solution) {
                            true => return true,
                            false => {
                                solution.pop();
                            }
                        }
                    }
                    false => (),
                }
            }
            false
        }

        pub fn n_queen_find_all_solutions(n: usize) -> Option<Vec<Vec<(usize, usize)>>> {
            let mut solutions = Vec::new();
            let mut current_solution = Vec::new();
            n_queen_all_solutions(n, 0, &mut current_solution, &mut solutions);
            match solutions.len() {
                0 => None,
                _ => Some(solutions),
            }
        }
        fn n_queen_all_solutions(
            n: usize,
            row: usize,
            current_solution: &mut Vec<(usize, usize)>,
            solutions: &mut Vec<Vec<(usize, usize)>>,
        ) {
            if row == n {
                solutions.push(current_solution.clone());
                return;
            }
            for col in 0..n {
                match n_queen_safe((col, row), current_solution) {
                    true => {
                        current_solution.push((col, row));
                        n_queen_all_solutions(n, row + 1, current_solution, solutions);
                        current_solution.pop();
                    }
                    false => (),
                }
            }
        }

        fn n_queen_safe(position: (usize, usize), solution: &[(usize, usize)]) -> bool {
            for other_position in solution {
                if other_position.0 == position.0
                    || other_position.1 == position.1
                    || (other_position.0.abs_diff(position.0)
                        == other_position.1.abs_diff(position.1))
                {
                    return false;
                }
            }
            true
        }
    }

    pub mod grid_ways {
        use std::collections::HashMap;

        pub fn count_paths_to_end(width: usize, height: usize) -> u64 {
            if width == 0 || height == 0 {
                return 0;
            }
            if width == 1 && height == 1 {
                return 1;
            }
            count_paths_to_end(width - 1, height) + count_paths_to_end(width, height - 1)
        }

        pub fn count_paths_to_end_fast(width: usize, height: usize) -> u128 {
            let mut memo = HashMap::new();
            count_paths_to_end_memoized(width, height, &mut memo)
        }
        pub fn count_paths_to_end_memoized(
            width: usize,
            height: usize,
            memo: &mut HashMap<(usize, usize), u128>,
        ) -> u128 {
            if width == 0 || height == 0 {
                return 0;
            }
            if width == 1 && height == 1 {
                return 1;
            }
            if let Some(memoized_result) = memo.get(&(width, height)) {
                return *memoized_result;
            }
            if let Some(memoized_result) = memo.get(&(height, width)) {
                return *memoized_result;
            }
            let result = count_paths_to_end_memoized(width - 1, height, memo)
                + count_paths_to_end_memoized(width, height - 1, memo);
            memo.insert((width, height), result);
            result
        }
    }

    pub mod sudoku {
        use super::super::print_2d::Vec2d;

        pub fn solve_sudoku(board: &mut Vec2d<u8>) -> bool {
            assert!(
                board.width() == 9 && board.height() == 9,
                "board must be 9x9"
            );
            for row in 0..=8 {
                for col in 0..=8 {
                    assert!(
                        board[(col, row)] <= 9,
                        "all values must be in range 0-9 inclusive"
                    );
                }
            }
            solve_sudoku_rec(board, 0, 0)
        }
        fn solve_sudoku_rec(board: &mut Vec2d<u8>, mut col: usize, mut row: usize) -> bool {
            while row < 9 && board[(col, row)] > 0 {
                (col, row) = next_col_row(col, row);
            }
            if row == 9 {
                return true;
            }
            for proposed_value in 1..=9 {
                if can_play(board, col, row, proposed_value) {
                    board[(col, row)] = proposed_value;
                    let (next_col, next_row) = next_col_row(col, row);
                    if solve_sudoku_rec(board, next_col, next_row) {
                        return true;
                    }
                    board[(col, row)] = 0;
                }
            }
            false
        }
        fn next_col_row(mut col: usize, mut row: usize) -> (usize, usize) {
            col += 1;
            if col > 8 {
                col = 0;
                row += 1;
            }
            (col, row)
        }
        fn can_play(board: &Vec2d<u8>, col: usize, row: usize, proposed_value: u8) -> bool {
            can_play_row(board, row, proposed_value)
                && can_play_column(board, col, proposed_value)
                && can_play_subgrid(board, col, row, proposed_value)
        }
        fn can_play_row(board: &Vec2d<u8>, row: usize, proposed_value: u8) -> bool {
            for col in 0..9 {
                if board[(col, row)] == proposed_value {
                    return false;
                }
            }
            true
        }
        fn can_play_column(board: &Vec2d<u8>, col: usize, proposed_value: u8) -> bool {
            for row in 0..9 {
                if board[(col, row)] == proposed_value {
                    return false;
                }
            }
            true
        }
        fn can_play_subgrid(board: &Vec2d<u8>, col: usize, row: usize, proposed_value: u8) -> bool {
            let (subgrid_col_start, subgrid_row_start) = subgrid_start(col, row);
            for row in subgrid_row_start..(subgrid_row_start + 3) {
                for col in subgrid_col_start..(subgrid_col_start + 3) {
                    if board[(col, row)] == proposed_value {
                        return false;
                    }
                }
            }
            true
        }
        fn subgrid_start(col: usize, row: usize) -> (usize, usize) {
            // with truncating integer division this does what is needed
            (col / 3 * 3, row / 3 * 3)
        }
    }

    pub mod rat_maze {
        use super::super::print_2d::Vec2d;

        pub fn find_path_to_end(maze: Vec2d<bool>) -> Option<Vec<(usize, usize)>> {
            let mut solution = Vec::new();
            solution.push((0, 0));
            if find_path_to_end_rec(&maze, 0, 0, &mut solution) {
                Some(solution)
            } else {
                None
            }
        }
        fn find_path_to_end_rec(
            maze: &Vec2d<bool>,
            col: usize,
            row: usize,
            solution: &mut Vec<(usize, usize)>,
        ) -> bool {
            if col >= maze.width() || row >= maze.height() {
                return false;
            }
            if col == maze.width() - 1 && row == maze.height() - 1 {
                return true;
            }
            for (next_col, next_row) in possible_next_positions(maze, col, row, solution) {
                solution.push((next_col, next_row));
                if find_path_to_end_rec(maze, next_col, next_row, solution) {
                    return true;
                }
                solution.pop();
            }
            false
        }
        fn possible_next_positions(
            maze: &Vec2d<bool>,
            col: usize,
            row: usize,
            solution: &[(usize, usize)],
        ) -> Vec<(usize, usize)> {
            let (maze_width, maze_height) = (maze.width(), maze.height());
            (0..4)
                .into_iter()
                .filter_map(move |rpn| relative_position(rpn, col, row, maze_width, maze_height))
                .filter(|position| is_not_blocked(position, maze))
                .filter(|position| is_not_already_visited(position, solution))
                .collect()
        }
        fn relative_position(
            position_number: u8,
            col: usize,
            row: usize,
            maze_width: usize,
            maze_height: usize,
        ) -> Option<(usize, usize)> {
            match position_number {
                0 if row > 0 => Some((col, row - 1)),
                1 if col < maze_width - 1 => Some((col + 1, row)),
                2 if row < maze_height - 1 => Some((col, row + 1)),
                3 if col > 0 => Some((col - 1, row)),
                _ => None,
            }
        }
        fn is_not_blocked(position: &(usize, usize), maze: &Vec2d<bool>) -> bool {
            !maze[*position]
        }
        fn is_not_already_visited(position: &(usize, usize), solution: &[(usize, usize)]) -> bool {
            !solution.iter().any(|pos| *pos == *position)
        }
    }

    pub mod word_break {
        use std::collections::HashSet;

        pub fn break_into_sentences(s: &str, dictionary: HashSet<&str>) -> Vec<String> {
            let mut solutions = Vec::new();
            let mut current_sentence = Vec::new();
            break_into_sentences_rec(
                &s.chars().collect::<Vec<_>>(),
                0,
                &dictionary,
                &mut solutions,
                &mut current_sentence,
            );
            solutions
        }
        fn break_into_sentences_rec(
            s: &[char],
            at: usize,
            dictionary: &HashSet<&str>,
            solutions: &mut Vec<String>,
            current_sentence: &mut Vec<String>,
        ) {
            if at == s.len() {
                solutions.push(current_sentence.join(" "));
                return;
            }
            let mut word = String::new();
            for c in s[at..].iter() {
                word.push(*c);
                if dictionary.contains(word.as_str()) {
                    current_sentence.push(word.clone());
                    break_into_sentences_rec(
                        s,
                        at + word.len(),
                        dictionary,
                        solutions,
                        current_sentence,
                    );
                    current_sentence.pop();
                }
            }
        }
    }

    pub mod power_set {
        use std::{
            collections::{HashSet, VecDeque},
            hash::Hash,
        };

        pub fn power_set<T: Copy + Eq + Hash>(set: &[T]) -> HashSet<VecDeque<T>> {
            let mut power_set = HashSet::new();
            power_set.insert(VecDeque::new());
            for i in 0..set.len() {
                for subset_length in 1..=set.len() - i {
                    power_set.extend(&mut power_set_rec(&set[i..], subset_length).into_iter());
                }
            }
            power_set
        }
        fn power_set_rec<T: Copy>(set: &[T], subset_length: usize) -> Vec<VecDeque<T>> {
            if subset_length == 0 {
                return Vec::from([VecDeque::new()]);
            }
            power_set_rec(&set[1..], subset_length - 1)
                .into_iter()
                .map(|mut v| {
                    v.push_front(set[0]);
                    v
                })
                .collect()
        }
    }

    pub mod word_search {
        use std::collections::HashSet;

        use super::super::print_2d::Vec2d;

        pub fn find_word(puzzle: &Vec2d<char>, word: &str) -> bool {
            let word = word.chars().collect::<Vec<_>>();
            let mut visited = HashSet::new();
            for row in 0..puzzle.height() {
                for col in 0..puzzle.width() {
                    if find_word_rec(puzzle, &word, col, row, &mut visited) {
                        return true;
                    }
                }
            }
            false
        }
        fn find_word_rec(
            puzzle: &Vec2d<char>,
            word: &[char],
            col: usize,
            row: usize,
            visited: &mut HashSet<(usize, usize)>,
        ) -> bool {
            if word.is_empty() {
                return true;
            }
            if puzzle[(col, row)] == word[0] {
                for (next_col, next_row) in
                    possible_moves(col, row, puzzle.width(), puzzle.height(), visited)
                        .into_iter()
                        .flatten()
                {
                    if find_word_rec(puzzle, &word[1..], next_col, next_row, visited) {
                        return true;
                    }
                }
            }
            false
        }
        fn possible_moves(
            col: usize,
            row: usize,
            width: usize,
            height: usize,
            visited: &HashSet<(usize, usize)>,
        ) -> [Option<(usize, usize)>; 4] {
            let mut moves = [None; 4];
            [
                MoveDirection::Up,
                MoveDirection::Down,
                MoveDirection::Left,
                MoveDirection::Right,
            ]
            .into_iter()
            .enumerate()
            .map(|(i, direction)| {
                (
                    i,
                    possible_move(direction, col, row, width, height, visited),
                )
            })
            .for_each(|(i, next_move)| moves[i] = next_move);
            moves
        }
        fn possible_move(
            direction: MoveDirection,
            col: usize,
            row: usize,
            width: usize,
            height: usize,
            visited: &HashSet<(usize, usize)>,
        ) -> Option<(usize, usize)> {
            match direction {
                MoveDirection::Up if row > 0 => Some((col, row - 1)),
                MoveDirection::Down if row < height - 1 => Some((col, row + 1)),
                MoveDirection::Left if col > 0 => Some((col - 1, row)),
                MoveDirection::Right if col < width - 1 => Some((col + 1, row)),
                _ => None,
            }
            .filter(|possible_move| !visited.contains(possible_move))
        }
        enum MoveDirection {
            Up,
            Down,
            Left,
            Right,
        }
    }
}

pub mod linked_list {
    use std::ptr::null;

    use self::{
        cursor::{CursorStart, LinkedListCursor},
        iter::LinkedListIterator,
        node::Node,
    };

    pub struct LinkedList<T> {
        head: *const Node<T>,
        tail: *const Node<T>,
        len: usize,
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            Self {
                head: null(),
                tail: null(),
                len: 0,
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn iter(&self) -> LinkedListIterator<&T, T> {
            LinkedListIterator::from(self)
        }

        pub fn iter_mut(&mut self) -> LinkedListIterator<&mut T, T> {
            LinkedListIterator::from(self)
        }

        pub fn cursor(&self, start: CursorStart) -> LinkedListCursor<T, &Self> {
            LinkedListCursor::from((self, start))
        }

        pub fn cursor_mut(&mut self, start: CursorStart) -> LinkedListCursor<T, &mut Self> {
            LinkedListCursor::from((self, start))
        }

        pub fn push_head(&mut self, item: T) {
            self.debug_panic_on_invalid_invariants();
            let old_head = self.head;
            self.head = Box::into_raw(Box::new(Node::new(item)));
            // SAFETY:
            //   - self.head is valid because it came from Box::into_raw
            //   - old_head is valid because self.head is valid by the invariants of Self
            unsafe { (*(self.head as *mut Node<T>)).set_next(old_head) }
            if old_head.is_null() {
                self.tail = self.head;
            } else {
                // SAFETY:
                //   - self.head is from Box::into_raw and so is valid
                //   - self.head.next is valid because old_head is valid as part of the invariants of Self
                //   - self.head.next.prev() would be null because it was the head Node
                unsafe { (*((*self.head).next() as *mut Node<T>)).set_prev(self.head) }
            }
            self.len += 1;
            self.debug_panic_on_invalid_invariants();
        }

        pub fn push_tail(&mut self, item: T) {
            self.debug_panic_on_invalid_invariants();
            let old_tail = self.tail;
            self.tail = Box::into_raw(Box::new(Node::new(item)));
            // SAFETY:
            //   - self.tail is valid because it came from Box::into_raw
            //   - old_tail is valid because because self.tail is valid by invariants of Self
            unsafe { (*(self.tail as *mut Node<T>)).set_prev(old_tail) }
            if old_tail.is_null() {
                self.head = self.tail;
            } else {
                // SAFETY:
                //   - self.head is from Box::into_raw and so is valid
                //   - self.head.next is valid because old_head is valid as part of the invariants of Self
                //   - self.head.next.prev() would be null because it was the head Node
                unsafe { (*((*self.tail).prev() as *mut Node<T>)).set_next(self.tail) }
            }
            self.len += 1;
            self.debug_panic_on_invalid_invariants();
        }

        pub fn pop_head(&mut self) -> Option<T> {
            self.debug_panic_on_invalid_invariants();
            let rv = if self.is_empty() || self.head.is_null() {
                None
            } else {
                let existing_head = self.head;
                // SAFETY:
                //   - existing_head is not null
                //   - existing_head is valid because it came from self.head which is by invariants of Self valid
                if unsafe { existing_head.as_ref().unwrap() }.next().is_null() {
                    self.head = null();
                    self.tail = null();
                } else {
                    // SAFETY:
                    //   - existing_head is not null
                    //   - existing_head is valid because it came from self.head which is by invariants of Self valid
                    self.head = unsafe { existing_head.as_ref().unwrap() }.next();
                }
                self.len -= 1;
                // SAFETY:
                //   - existing head is not null and is valid (as above)
                Some(unsafe { Box::from_raw(existing_head as *mut Node<T>) }.into_item())
            };
            self.debug_panic_on_invalid_invariants();
            rv
        }

        pub fn pop_tail(&mut self) -> Option<T> {
            self.debug_panic_on_invalid_invariants();
            let rv = if self.is_empty() || self.tail.is_null() {
                None
            } else {
                let existing_tail = self.tail;
                // SAFETY:
                //   - existing_tail is not null
                //   - existing_tail is valid because it came from self.tail which is by invariants of Self valid
                if unsafe { existing_tail.as_ref().unwrap() }.prev().is_null() {
                    self.head = null();
                    self.tail = null();
                } else {
                    // SAFETY:
                    //   - existing_tail is not null
                    //   - existing_tail is valid because it came from self.tail which is by invariants of Self valid
                    self.tail = unsafe { existing_tail.as_ref().unwrap() }.prev();
                }
                self.len -= 1;
                // SAFETY:
                //   - existing tail is not null and is valid (as above)
                Some(unsafe { Box::from_raw(existing_tail as *mut Node<T>) }.into_item())
            };
            self.debug_panic_on_invalid_invariants();
            rv
        }

        pub fn peek_head(&self) -> Option<&T> {
            if self.head.is_null() {
                None
            } else {
                // SAFETY:
                //   - self.head is not null
                //   - and is a properly aligned/aollcated pointer as part of the invariants of the type
                unsafe { Some((*self.head).as_ref()) }
            }
        }

        pub fn peek_tail(&self) -> Option<&T> {
            if self.tail.is_null() {
                None
            } else {
                // SAFETY:
                //   - self.tail is not null
                //   - and is a properly aligned/aollcated pointer as part of the invariants of the type
                unsafe { Some((*self.tail).as_ref()) }
            }
        }

        fn debug_panic_on_invalid_invariants(&self) {
            debug_assert!(
                (self.len == 0 && self.head.is_null() && self.tail.is_null())
                    || (self.len == 1 && !self.head.is_null() && self.tail == self.head)
                    || (self.len > 1
                        && !self.head.is_null()
                        && !self.tail.is_null()
                        && self.head != self.tail)
            );
        }
    }

    impl<T> Default for LinkedList<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    pub mod iter {
        use std::marker::PhantomData;

        use super::node::Node;
        use super::LinkedList;

        pub struct LinkedListIterator<T, U> {
            front: *const Node<U>,
            back: *const Node<U>,
            _returning: PhantomData<T>,
        }

        impl<T, U> LinkedListIterator<T, U> {}

        impl<'a, T> From<&'a LinkedList<T>> for LinkedListIterator<&'a T, T> {
            fn from(list: &'a LinkedList<T>) -> Self {
                Self {
                    front: list.head,
                    back: list.tail,
                    _returning: PhantomData::default(),
                }
            }
        }

        impl<'a, T> From<&'a mut LinkedList<T>> for LinkedListIterator<&'a mut T, T> {
            fn from(list: &'a mut LinkedList<T>) -> Self {
                Self {
                    front: list.head,
                    back: list.tail,
                    _returning: PhantomData::default(),
                }
            }
        }

        impl<'a, T> Iterator for LinkedListIterator<&'a T, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.front.is_null() {
                    None
                } else {
                    // SAFETY:
                    //   - self.front is not null
                    //   - self.front is a valid pointer because it came from LinkedList.head or a subsequent
                    //     Node.next so the invariants of LinkedList ensure the validity of these pointers
                    //   - Node.next is a valid pointer because LinkedList invariant ensure that or it is null
                    //   - Nodes will not be removed/added while this iterator is active because it borrows the
                    //     LinkedList shared/immutably
                    let next = unsafe { self.front.as_ref() }.unwrap().next();
                    let rv = Some(unsafe { self.front.as_ref() }.unwrap().as_ref());
                    self.front = next;
                    rv
                }
            }
        }

        impl<'a, T> DoubleEndedIterator for LinkedListIterator<&'a T, T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.back.is_null() {
                    None
                } else {
                    // SAFETY:
                    //   - self.back is not null
                    //   - self.back is a valid pointer because it came from LinkedList.head or a subsequent
                    //     Node.prev so the invariants of LinkedList ensure the validity of these pointers
                    //   - Node.prev is a valid pointer because LinkedList invariant ensure that or it is null
                    //   - Nodes will not be removed/added while this iterator is active because it borrows the
                    //     LinkedList shared/immutably
                    let prev = unsafe { self.back.as_ref() }.unwrap().prev();
                    let rv = Some(unsafe { self.back.as_ref() }.unwrap().as_ref());
                    self.back = prev;
                    rv
                }
            }
        }

        impl<'a, T> Iterator for LinkedListIterator<&'a mut T, T> {
            type Item = &'a mut T;

            fn next(&mut self) -> Option<Self::Item> {
                if self.front.is_null() {
                    None
                } else {
                    // SAFETY:
                    //   - self.front is not null
                    //   - self.front is a valid pointer because it came from LinkedList.head or a subsequent
                    //     Node.next so the invariants of LinkedList ensure the validity of these pointers
                    //   - Node.next is a valid pointer because LinkedList invariant ensure that or it is null
                    //   - Nodes will not be removed/added while this iterator is active because it borrows the
                    //     LinkedList shared/immutably
                    let next = unsafe { self.front.as_ref() }.unwrap().next();
                    let rv = Some(
                        unsafe { (self.front as *mut Node<T>).as_mut() }
                            .unwrap()
                            .as_mut(),
                    );
                    self.front = next;
                    rv
                }
            }
        }

        impl<'a, T> DoubleEndedIterator for LinkedListIterator<&'a mut T, T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.back.is_null() {
                    None
                } else {
                    // SAFETY:
                    //   - self.back is not null
                    //   - self.back is a valid pointer because it came from LinkedList.head or a subsequent
                    //     Node.prev so the invariants of LinkedList ensure the validity of these pointers
                    //   - Node.prev is a valid pointer because LinkedList invariant ensure that or it is null
                    //   - Nodes will not be removed/added while this iterator is active because it borrows the
                    //     LinkedList shared/immutably
                    let prev = unsafe { self.back.as_ref() }.unwrap().prev();
                    let rv = Some(
                        unsafe { (self.back as *mut Node<T>).as_mut() }
                            .unwrap()
                            .as_mut(),
                    );
                    self.back = prev;
                    rv
                }
            }
        }
    }

    pub mod cursor {
        use std::cmp::Ordering;
        use std::ptr::null;

        use super::{LinkedList, Node};

        pub struct LinkedListCursor<T, L> {
            list: L,
            at: Option<usize>,
            current: *const Node<T>,
        }

        impl<'a, T> From<(&'a LinkedList<T>, CursorStart)> for LinkedListCursor<T, &'a LinkedList<T>> {
            fn from((list, start_at): (&'a LinkedList<T>, CursorStart)) -> Self {
                let (at, current) = cursor_starting_values(start_at, list);
                Self {
                    list,
                    at,
                    current: current.unwrap_or(null()),
                }
            }
        }

        impl<'a, T> From<(&'a mut LinkedList<T>, CursorStart)>
            for LinkedListCursor<T, &'a mut LinkedList<T>>
        {
            fn from((list, start_at): (&'a mut LinkedList<T>, CursorStart)) -> Self {
                let (at, current) = cursor_starting_values(start_at, list);
                Self {
                    list,
                    at,
                    current: current.unwrap_or(null()),
                }
            }
        }

        trait LinkedListCursorAccess<T> {
            fn head(&self) -> *const Node<T>;
            fn tail(&self) -> *const Node<T>;
            fn len(&self) -> usize;
            fn at(&self) -> Option<usize>;
            fn current(&self) -> *const Node<T>;
            fn set_at(&mut self, value: Option<usize>);
            fn set_current(&mut self, value: *const Node<T>);
        }

        trait LinkedListCursorAccessMut<T>: LinkedListCursorAccess<T> {
            fn set_head(&mut self, value: *const Node<T>);
            fn set_tail(&mut self, value: *const Node<T>);
            fn len_mut(&mut self) -> &mut usize;
        }

        impl<'a, T> LinkedListCursorAccess<T> for LinkedListCursor<T, &'a LinkedList<T>> {
            fn head(&self) -> *const Node<T> {
                self.list.head
            }
            fn tail(&self) -> *const Node<T> {
                self.list.tail
            }
            fn len(&self) -> usize {
                self.list.len()
            }
            fn at(&self) -> Option<usize> {
                self.at
            }
            fn current(&self) -> *const Node<T> {
                self.current
            }
            fn set_at(&mut self, value: Option<usize>) {
                self.at = value
            }
            fn set_current(&mut self, value: *const Node<T>) {
                self.current = value
            }
        }

        impl<'a, T> LinkedListCursorAccess<T> for LinkedListCursor<T, &'a mut LinkedList<T>> {
            fn head(&self) -> *const Node<T> {
                self.list.head
            }
            fn tail(&self) -> *const Node<T> {
                self.list.tail
            }
            fn len(&self) -> usize {
                self.list.len()
            }
            fn at(&self) -> Option<usize> {
                self.at
            }
            fn current(&self) -> *const Node<T> {
                self.current
            }
            fn set_at(&mut self, value: Option<usize>) {
                self.at = value
            }
            fn set_current(&mut self, value: *const Node<T>) {
                self.current = value
            }
        }

        impl<'a, T> LinkedListCursorAccessMut<T> for LinkedListCursor<T, &'a mut LinkedList<T>> {
            fn set_head(&mut self, value: *const Node<T>) {
                self.list.head = value;
            }
            fn set_tail(&mut self, value: *const Node<T>) {
                self.list.tail = value;
            }
            fn len_mut(&mut self) -> &mut usize {
                &mut self.list.len
            }
        }

        pub enum CursorStart {
            Head,
            Tail,
        }

        pub trait Cursor<T> {
            fn list_len(&self) -> usize;
            fn list_is_empty(&self) -> bool;
            fn on_item(&self) -> bool;
            fn at(&self) -> Option<usize>;
            fn seek_head(&mut self) -> Result<(), error::CursorPositionError>;
            fn seek_tail(&mut self) -> Result<(), error::CursorPositionError>;
            fn seek(&mut self, to: usize) -> Result<(), error::CursorPositionError>;
            fn seek_prev(&mut self, to: usize) -> Result<(), error::CursorPositionError>;
            fn seek_next(&mut self, to: usize) -> Result<(), error::CursorPositionError>;
            fn prev(&mut self) -> Result<(), error::CursorPositionError>;
            fn next(&mut self) -> Result<(), error::CursorPositionError>;
            fn peek(&self) -> Option<&T>;
            fn peek_prev(&self) -> Option<&T>;
            fn peek_next(&self) -> Option<&T>;
        }

        pub trait MutCursor<T>: Cursor<T> {
            fn insert_head(&mut self, item: T);
            fn insert_tail(&mut self, item: T);
            fn insert_before(&mut self, item: T) -> Result<(), error::CursorPositionError>;
            fn insert_after(&mut self, item: T) -> Result<(), error::CursorPositionError>;
            fn remove(&mut self) -> Result<T, error::CursorPositionError>;
            fn replace(&mut self, item: T) -> Result<T, error::CursorPositionError>;
            fn peek_mut(&mut self) -> Option<&mut T>;
            fn peek_prev_mut(&mut self) -> Option<&mut T>;
            fn peek_next_mut(&mut self) -> Option<&mut T>;
        }

        impl<'a, T> Cursor<T> for LinkedListCursor<T, &'a LinkedList<T>> {
            fn list_len(&self) -> usize {
                self.list.len()
            }
            fn list_is_empty(&self) -> bool {
                self.list.is_empty()
            }
            fn on_item(&self) -> bool {
                self.at.is_some()
            }
            fn at(&self) -> Option<usize> {
                self.at
            }
            fn seek_head(&mut self) -> Result<(), error::CursorPositionError> {
                seek_head(self)
            }
            fn seek_tail(&mut self) -> Result<(), error::CursorPositionError> {
                seek_tail(self)
            }
            fn seek(&mut self, to: usize) -> Result<(), error::CursorPositionError> {
                if to >= self.list_len() {
                    Err(error::CursorPositionError::new(
                        error::CursorPositionErrorKind::ExceededLength,
                    ))
                } else {
                    if self.at.is_none() {
                        let _ = self.seek_head();
                    }
                    match self.at {
                        Some(at) => match at.cmp(&to) {
                            Ordering::Equal => Ok(()),
                            Ordering::Less => self.seek_next(to - at),
                            Ordering::Greater => self.seek_prev(at - to),
                        },
                        None => Err(error::CursorPositionError::new(
                            error::CursorPositionErrorKind::IsEmpty,
                        )),
                    }
                }
            }
            fn seek_prev(&mut self, to: usize) -> Result<(), error::CursorPositionError> {
                seek_prev(self, to)
            }
            fn seek_next(&mut self, to: usize) -> Result<(), error::CursorPositionError> {
                seek_next(self, to)
            }
            fn prev(&mut self) -> Result<(), error::CursorPositionError> {
                self.seek_prev(1)
            }
            fn next(&mut self) -> Result<(), error::CursorPositionError> {
                self.seek_next(1)
            }
            fn peek(&self) -> Option<&T> {
                peek(self)
            }
            fn peek_prev(&self) -> Option<&T> {
                peek_prev(self)
            }
            fn peek_next(&self) -> Option<&T> {
                peek_next(self)
            }
        }

        impl<'a, T> Cursor<T> for LinkedListCursor<T, &'a mut LinkedList<T>> {
            fn list_len(&self) -> usize {
                self.list.len()
            }
            fn list_is_empty(&self) -> bool {
                self.list.is_empty()
            }
            fn on_item(&self) -> bool {
                self.at.is_some()
            }
            fn at(&self) -> Option<usize> {
                self.at
            }
            fn seek_head(&mut self) -> Result<(), error::CursorPositionError> {
                seek_head(self)
            }
            fn seek_tail(&mut self) -> Result<(), error::CursorPositionError> {
                seek_tail(self)
            }
            fn seek(&mut self, to: usize) -> Result<(), error::CursorPositionError> {
                if to >= self.list_len() {
                    Err(error::CursorPositionError::new(
                        error::CursorPositionErrorKind::ExceededLength,
                    ))
                } else {
                    if self.at.is_none() {
                        let _ = self.seek_head();
                    }
                    match self.at {
                        Some(at) => match at.cmp(&to) {
                            Ordering::Equal => Ok(()),
                            Ordering::Less => self.seek_next(to - at),
                            Ordering::Greater => self.seek_prev(at - to),
                        },
                        None => Err(error::CursorPositionError::new(
                            error::CursorPositionErrorKind::IsEmpty,
                        )),
                    }
                }
            }
            fn seek_prev(&mut self, to: usize) -> Result<(), error::CursorPositionError> {
                seek_prev(self, to)
            }
            fn seek_next(&mut self, to: usize) -> Result<(), error::CursorPositionError> {
                seek_next(self, to)
            }
            fn prev(&mut self) -> Result<(), error::CursorPositionError> {
                self.seek_prev(1)
            }
            fn next(&mut self) -> Result<(), error::CursorPositionError> {
                self.seek_next(1)
            }
            fn peek(&self) -> Option<&T> {
                peek(self)
            }
            fn peek_prev(&self) -> Option<&T> {
                peek_prev(self)
            }
            fn peek_next(&self) -> Option<&T> {
                peek_next(self)
            }
        }

        impl<'a, T> MutCursor<T> for LinkedListCursor<T, &'a mut LinkedList<T>> {
            fn insert_head(&mut self, item: T) {
                self.list.push_head(item);
                if let Some(ref mut at) = self.at {
                    *at += 1;
                }
            }
            fn insert_tail(&mut self, item: T) {
                self.list.push_tail(item);
            }
            fn insert_before(&mut self, item: T) -> Result<(), error::CursorPositionError> {
                if let Some(ref mut at) = self.at {
                    // SAFETY:
                    //    - by invariants of type if self.at is not None then self.current is a valid, non-null ptr
                    let old_prev = unsafe { self.current.as_ref() }.unwrap().prev();
                    let new_node = Box::into_raw(Box::new(Node::new(item)));
                    // SAFETY:
                    //   - new_node is non-null and valid because it came from Box::into_raw
                    unsafe { (new_node as *mut Node<T>).as_mut() }
                        .unwrap()
                        .set_next(self.current);
                    // SAFETY:
                    //   - self.current is not null and is a valid ptr when self.at is Some by type invariants
                    //   - new_node is non-null and valid because it came from Box::into_raw
                    unsafe { (self.current as *mut Node<T>).as_mut() }
                        .unwrap()
                        .set_prev(new_node as *const Node<T>);
                    if !old_prev.is_null() {
                        // SAFETY:
                        //   - new_node is non-null and valid as above
                        //   - old_prev is valid and non-null value of self.current.prev before changes
                        unsafe { (new_node as *mut Node<T>).as_mut() }
                            .unwrap()
                            .set_prev(old_prev);
                        unsafe { (old_prev as *mut Node<T>).as_mut() }
                            .unwrap()
                            .set_next(new_node as *const Node<T>);
                    } else {
                        self.list.head = new_node;
                    }
                    self.list.len += 1;
                    *at += 1;
                    Ok(())
                } else {
                    Err(error::CursorPositionError::new(
                        error::CursorPositionErrorKind::NotOnItem,
                    ))
                }
            }
            fn insert_after(&mut self, item: T) -> Result<(), error::CursorPositionError> {
                if self.at.is_some() {
                    // SAFETY:
                    //    - by invariants of type if self.at is not None then self.current is a valid, non-null ptr
                    let old_next = unsafe { self.current.as_ref() }.unwrap().next();
                    let new_node = Box::into_raw(Box::new(Node::new(item)));
                    // SAFETY:
                    //   - new_node is non-null and valid because it came from Box::into_raw
                    unsafe { (new_node as *mut Node<T>).as_mut() }
                        .unwrap()
                        .set_prev(self.current);
                    // SAFETY:
                    //   - self.current is not null and is a valid ptr when self.at is Some by type invariants
                    //   - new_node is non-null and valid because it came from Box::into_raw
                    unsafe { (self.current as *mut Node<T>).as_mut() }
                        .unwrap()
                        .set_next(new_node as *const Node<T>);
                    if !old_next.is_null() {
                        // SAFETY:
                        //   - new_node is non-null and valid as above
                        //   - old_next is valid and non-null value of self.current.next before changes
                        unsafe { (new_node as *mut Node<T>).as_mut() }
                            .unwrap()
                            .set_next(old_next);
                        unsafe { (old_next as *mut Node<T>).as_mut() }
                            .unwrap()
                            .set_next(new_node as *const Node<T>);
                    } else {
                        self.list.tail = new_node;
                    }
                    self.list.len += 1;
                    Ok(())
                } else {
                    Err(error::CursorPositionError::new(
                        error::CursorPositionErrorKind::NotOnItem,
                    ))
                }
            }
            fn remove(&mut self) -> Result<T, error::CursorPositionError> {
                if self.at.is_some() {
                    let current = self.current;
                    // SAFETY:
                    //   - current is not null and valid because it came from
                    //     self.current which is valid/not null when self.at.is_some()
                    let prev = unsafe { current.as_ref() }.unwrap().prev();
                    let next = unsafe { current.as_ref() }.unwrap().next();
                    if !prev.is_null() {
                        // SAFETY:
                        //   - prev is not null and due to invariants of List and Cursor types is valid
                        unsafe { (prev as *mut Node<T>).as_mut() }
                            .unwrap()
                            .set_next(next);
                    } else {
                        self.list.head = next;
                    }
                    if !next.is_null() {
                        // SAFETY:
                        //   - next is not null and due to invariants of List and Cursor types is valid
                        unsafe { (next as *mut Node<T>).as_mut() }
                            .unwrap()
                            .set_prev(prev);
                    } else {
                        self.list.tail = prev;
                    }
                    *self.len_mut() -= 1;
                    (self.at, self.current) = if prev.is_null() && next.is_null() {
                        (None, null())
                    } else if next.is_null() {
                        (Some(self.at.unwrap() - 1), prev)
                    } else {
                        (self.at, next)
                    };
                    // SAFETY:
                    //   - current is not null and valid due to self.current being not null and valid as per invariants when self.at.is_some()
                    Ok(unsafe { Box::from_raw(current as *mut Node<T>) }.into_item())
                } else {
                    Err(error::CursorPositionError::new(
                        error::CursorPositionErrorKind::NotOnItem,
                    ))
                }
            }
            fn replace(&mut self, item: T) -> Result<T, error::CursorPositionError> {
                if self.at.is_some() {
                    // SAFETY:
                    //   - self.current is non-null and valid when self.at is Some by type invariants
                    //   - replace_item will use mem::replace to swap the item in the node
                    //   - we have mutable/exlusive access to the cursor which has mutable/exclusive
                    //     access to the list
                    Ok(unsafe { (self.current as *mut Node<T>).as_mut() }
                        .unwrap()
                        .replace_item(item))
                } else {
                    Err(error::CursorPositionError::new(
                        error::CursorPositionErrorKind::NotOnItem,
                    ))
                }
            }
            fn peek_mut(&mut self) -> Option<&mut T> {
                if self.at.is_some() {
                    // SAFETY:
                    //   - self.current is non-null and valid when self.at is Some by type invariants
                    //   - the mutable reference returned cannot outlive the lifetime of the cursor
                    //     and will prevent further cursor methods from being called until the borrow ends
                    Some(
                        unsafe { (self.current as *mut Node<T>).as_mut() }
                            .unwrap()
                            .item_mut(),
                    )
                } else {
                    None
                }
            }
            fn peek_prev_mut(&mut self) -> Option<&mut T> {
                if self.at.is_some() {
                    // SAFETY:
                    //   - self.current is non-null and valid when self.at is Some by type invariants
                    //   - the mutable reference returned cannot outlive the lifetime of the cursor
                    //     and will prevent further cursor methods from being called until the borrow ends
                    if !unsafe { self.current.as_ref() }.unwrap().prev().is_null() {
                        Some(
                            unsafe {
                                (self.current.as_ref().unwrap().prev() as *mut Node<T>).as_mut()
                            }
                            .unwrap()
                            .item_mut(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            fn peek_next_mut(&mut self) -> Option<&mut T> {
                if self.at.is_some() {
                    // SAFETY:
                    //   - self.current is non-null and valid when self.at is Some by type invariants
                    //   - the mutable reference returned cannot outlive the lifetime of the cursor
                    //     and will prevent further cursor methods from being called until the borrow ends
                    if !unsafe { self.current.as_ref() }.unwrap().next().is_null() {
                        Some(
                            unsafe {
                                (self.current.as_ref().unwrap().next() as *mut Node<T>).as_mut()
                            }
                            .unwrap()
                            .item_mut(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }

        pub mod error {
            #[derive(Debug, Eq, PartialEq)]
            pub struct CursorPositionError {
                kind: CursorPositionErrorKind,
            }

            impl CursorPositionError {
                pub fn new(kind: CursorPositionErrorKind) -> Self {
                    Self { kind }
                }
                pub fn kind(&self) -> &CursorPositionErrorKind {
                    &self.kind
                }
            }

            #[derive(Debug, Eq, PartialEq)]
            pub enum CursorPositionErrorKind {
                ExceededLength,
                IsEmpty,
                NotOnItem,
                AttemptToPositionBeforeHead,
                AttemptToPositionAfterTail,
            }
        }

        fn cursor_starting_values<T>(
            start_at: CursorStart,
            list: &LinkedList<T>,
        ) -> (Option<usize>, Option<*const Node<T>>) {
            let (at, current) = match start_at {
                CursorStart::Head => (
                    if list.head.is_null() { None } else { Some(0) },
                    if list.head.is_null() {
                        None
                    } else {
                        Some(list.head)
                    },
                ),
                CursorStart::Tail => (
                    if list.tail.is_null() {
                        None
                    } else {
                        Some(list.len() - 1)
                    },
                    if list.tail.is_null() {
                        None
                    } else {
                        Some(list.tail)
                    },
                ),
            };
            (at, current)
        }

        fn seek_head<T, C>(cursor: &mut C) -> Result<(), error::CursorPositionError>
        where
            C: LinkedListCursorAccess<T>,
        {
            cursor.set_current(cursor.head());
            cursor.set_at(if cursor.current().is_null() {
                None
            } else {
                Some(0)
            });
            if cursor.at().is_none() {
                Err(error::CursorPositionError::new(
                    error::CursorPositionErrorKind::IsEmpty,
                ))
            } else {
                Ok(())
            }
        }

        fn seek_tail<T, C>(cursor: &mut C) -> Result<(), error::CursorPositionError>
        where
            C: LinkedListCursorAccess<T>,
        {
            cursor.set_current(cursor.tail());
            cursor.set_at(if cursor.current().is_null() {
                None
            } else {
                Some(cursor.len() - 1)
            });
            if cursor.at().is_none() {
                Err(error::CursorPositionError::new(
                    error::CursorPositionErrorKind::IsEmpty,
                ))
            } else {
                Ok(())
            }
        }

        fn seek_prev<T, C>(cursor: &mut C, to: usize) -> Result<(), error::CursorPositionError>
        where
            C: LinkedListCursorAccess<T>,
        {
            if cursor.current().is_null() {
                Err(error::CursorPositionError::new(
                    error::CursorPositionErrorKind::NotOnItem,
                ))
            } else {
                for _ in 0..to {
                    let original_at = cursor.at().unwrap();
                    // SAFETY:
                    //   - cursor.current is not null
                    //   - cursor.current is a valid pointer due to invariants of LinkedList and Self
                    //   - cursor.current.prev() is either null or a valid pointer due to invariantes of LinkedList and Self
                    cursor.set_current(unsafe { (*cursor.current()).prev() });
                    if cursor.current().is_null() {
                        cursor.set_at(None);
                        return Err(error::CursorPositionError::new(
                            error::CursorPositionErrorKind::AttemptToPositionBeforeHead,
                        ));
                    } else {
                        cursor.set_at(Some(original_at - 1));
                    };
                }
                Ok(())
            }
        }

        fn seek_next<T, C>(cursor: &mut C, to: usize) -> Result<(), error::CursorPositionError>
        where
            C: LinkedListCursorAccess<T>,
        {
            if cursor.current().is_null() {
                Err(error::CursorPositionError::new(
                    error::CursorPositionErrorKind::NotOnItem,
                ))
            } else {
                for _ in 0..to {
                    let original_at = cursor.at().unwrap();
                    // SAFETY:
                    //   - cursor.current is not null
                    //   - cursor.current is a valid pointer due to invariants of LinkedList and cursor
                    //   - cursor.current.next() is either null or a valid pointer due to invariantes of LinkedList and cursor
                    cursor.set_current(unsafe { (*cursor.current()).next() });
                    if cursor.current().is_null() {
                        cursor.set_at(None);
                        return Err(error::CursorPositionError::new(
                            error::CursorPositionErrorKind::AttemptToPositionAfterTail,
                        ));
                    } else {
                        cursor.set_at(Some(original_at + 1));
                    }
                }
                Ok(())
            }
        }

        fn peek<T, C>(cursor: &C) -> Option<&T>
        where
            C: LinkedListCursorAccess<T>,
        {
            if cursor.current().is_null() {
                None
            } else {
                // SAFETY:
                //   - cursor.current is not null
                //   - cursor.current is a valid pointer due to invariants of LinkedList an LinkedListCursor
                Some(unsafe { cursor.current().as_ref() }.unwrap().item())
            }
        }

        fn peek_prev<T, C>(cursor: &C) -> Option<&T>
        where
            C: LinkedListCursorAccess<T>,
        {
            if cursor.current().is_null() {
                None
            } else {
                // SAFETY:
                //   - cursor.current is not null
                //   - cursor.current is a valid pointer due to invariants of LinkedList an LinkedListCursor
                //   - cursor.current.prev is null or is a valid pointer due to the LinkedList/Self invariants
                let prev = unsafe { cursor.current().as_ref() }.unwrap().prev();
                if prev.is_null() {
                    None
                } else {
                    Some(unsafe { prev.as_ref() }.unwrap().item())
                }
            }
        }

        fn peek_next<T, C>(cursor: &C) -> Option<&T>
        where
            C: LinkedListCursorAccess<T>,
        {
            if cursor.current().is_null() {
                None
            } else {
                // SAFETY:
                //   - cursor.current is not null
                //   - cursor.current is a valid pointer due to invariants of LinkedList an LinkedListCursor
                //   - cursor.current.next is null or is a valid pointer due to the LinkedList/Self invariants
                let next = unsafe { cursor.current().as_ref() }.unwrap().next();
                if next.is_null() {
                    None
                } else {
                    Some(unsafe { next.as_ref() }.unwrap().item())
                }
            }
        }
    }

    mod node {
        use std::{mem, ptr::null};

        pub struct Node<T> {
            prev: *const Node<T>,
            next: *const Node<T>,
            item: T,
        }

        impl<T> Node<T> {
            pub fn new(item: T) -> Self {
                Self {
                    prev: null(),
                    next: null(),
                    item,
                }
            }
            pub fn set_prev(&mut self, node: *const Node<T>) {
                self.prev = node;
            }
            pub fn prev(&self) -> *const Node<T> {
                self.prev
            }
            pub fn set_next(&mut self, node: *const Node<T>) {
                self.next = node;
            }
            pub fn next(&self) -> *const Node<T> {
                self.next
            }
            pub fn into_item(self) -> T {
                self.item
            }
            pub fn item(&self) -> &T {
                &self.item
            }
            pub fn replace_item(&mut self, item: T) -> T {
                mem::replace(&mut self.item, item)
            }
            pub fn item_mut(&mut self) -> &mut T {
                &mut self.item
            }
        }

        impl<T> AsRef<T> for Node<T> {
            fn as_ref(&self) -> &T {
                &self.item
            }
        }

        impl<T> AsMut<T> for Node<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.item
            }
        }
    }
}

pub mod stack {

    pub fn stock_spanner<T: Ord>(values: &[T]) -> Vec<usize> {
        let stack = Vec::new();
        let result = Vec::new();

        result
    }
}
