use super::individual::Individual;
use super::helpers;
use rand::Rng;

struct Crossover;
struct Select;
struct Mutate;

impl Crossover {
    fn one_point<T: Clone>(a: &Individual<T>, b: &Individual<T>, point_idx: Option<usize>) -> (Individual<T>, Individual<T>) {

        let idx = if let Some(val) = point_idx {
            val
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..a.value.len() - 1)
        };

        let mut values_left: Vec<T> = Vec::new();
        values_left.extend_from_slice(&a.value[..idx]);
        values_left.extend_from_slice(&b.value[idx..]);

        let mut values_right: Vec<T> = Vec::new();
        values_right.extend_from_slice(&b.value[..idx]);
        values_right.extend_from_slice(&a.value[idx..]);


        (
            Individual::new(values_left, None),
            Individual::new(values_right, None)
        )
    }

    fn two_points<T: Copy>(a: &Individual<T>, b: &Individual<T>, points_range: (Option<usize>, Option<usize>)) -> (Individual<T>, Individual<T>) {
        let (point_left, point_right) = helpers::process_two_points_or_generate(a.value.len(), points_range);

        let mut values_left: Vec<T> = Vec::new();
        values_left.extend_from_slice(&a.value[..point_left]);
        values_left.extend_from_slice(&b.value[point_left..point_right]);
        values_left.extend_from_slice(&a.value[point_right..]);

        let mut values_right: Vec<T> = Vec::new();
        values_right.extend_from_slice(&b.value[..point_left]);
        values_right.extend_from_slice(&a.value[point_left..point_right]);
        values_right.extend_from_slice(&b.value[point_right..]);

        (
            Individual::new(values_left, None),
            Individual::new(values_right, None),
        )
    }

    fn ordered<T: Clone + std::cmp::PartialEq>(a: &Individual<T>, b: &Individual<T>) -> (Individual<T>, Individual<T>) {
        let (point_left, point_right) = helpers::process_two_points_or_generate(a.value.len(), (None, None));
        let value_length = a.value.len();

        let mut child_a_value = vec![None; point_left];
        child_a_value.extend(b.value[point_left..point_right].iter().cloned().map(Some));
        child_a_value.extend(vec![None; value_length - point_right]);

        let mut child_b_value = vec![None; point_left];
        child_b_value.extend(a.value[point_left..point_right].iter().cloned().map(Some));
        child_b_value.extend(vec![None; value_length - point_right]);

        let run = |ind: &Individual<T>, arr: &mut Vec<Option<T>>| {
            let arr_len = arr.len();
            let mut curr_idx = point_right % arr_len;
            let mut arr_idx = 0;
            loop {
                if arr.iter().all(|el| el.is_some()) {
                    break
                }
                // if arr.contains(Some(*ind.value[arr_idx])) {
                if arr.iter().flatten().any(|el| ind.value[arr_idx] == *el) {
                    arr_idx = (arr_idx + 1) % arr_len;
                } else {
                    arr[curr_idx] = Some(ind.value[arr_idx].clone());
                    curr_idx = (curr_idx + 1) + arr_len;
                    arr_idx = (arr_idx + 1) % arr_len
                }
            }
        };

        run(a, &mut child_a_value);
        run(b, &mut child_b_value);

        (
            Individual::new(child_a_value.into_iter().filter_map(|el| el).collect(), None),
            Individual::new(child_b_value.into_iter().filter_map(|el| el).collect(), None)
        )
    }
}