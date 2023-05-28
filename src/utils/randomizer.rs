/* #region Imports */
// 3rd Party
use rand::Rng;
use rand::distributions::uniform::{SampleUniform, SampleRange};
/* #endregion */

pub fn get_random_number_in_range<T, R>(range: R) -> T
where T: SampleUniform, R: SampleRange<T> {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

fn get_random_index_from_array<T>(array: &[T]) -> usize {
    get_random_number_in_range(0..array.len())
}

pub fn get_random_value_from_array<T>(array: &[T]) -> &T {
    &array[get_random_index_from_array(array)]
}

pub fn get_random_value_from_array_as_mut<T>(array: &mut [T]) -> &mut T {
    &mut array[get_random_index_from_array(array)]
}