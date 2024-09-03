mod password_manager;

use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn jumping_on_clouds(clouds: &[i32]) -> i32 {
    let mut jumps = 0;

    let mut clouds_iter = clouds.iter();
    let mut next_cloud = clouds_iter.next();
    while let Some(&current_cloud) = next_cloud {
        if current_cloud == 1 {
            jumps += 1;
            continue;
        }

        next_cloud = clouds_iter.next();
    }

    jumps
}

fn repeated_string(s: &str, n: i64) -> i64 {
    let a_count = s.chars().filter(|&c| c == 'a').count() as i64;
    let string_size = s.len() as i64;

    if n % string_size == 0 {
        a_count * (n / string_size)
    } else {
        let remainder = (n % string_size) as usize;
        let leftover_count = s[..remainder].chars().filter(|&c| c == 'a').count() as i64;
        a_count * (n / string_size) + leftover_count
    }
}

fn hourglass_sum(arr: &[Vec<i32>]) -> i32 {
    let mut max_hourglass_sum = i32::MIN;

    for row in 0..arr.len() {
        for col in 0..arr.len() {
            if row > 3 || col > 3 {
                continue;
            }
            let hourglass_sum = arr[row][col]
                + arr[row][col + 1]
                + arr[row][col + 2]
                + arr[row + 1][col + 1]
                + arr[row + 2][col]
                + arr[row + 2][col + 1]
                + arr[row + 2][col + 2];
            if hourglass_sum > max_hourglass_sum {
                max_hourglass_sum = hourglass_sum;
            }
        }
    }
    max_hourglass_sum
}

fn rot_left(a: &[i32], d: i32) -> Vec<i32> {
    let d = d as usize;
    [&a[d..], &a[..d]].concat()
}

fn minimum_bribes(queue: &[i32]) {
    let mut bribes = 0;
    let queue: Vec<i32> = queue.iter().map(|e| e - 1).collect();

    for (index, &element) in queue.iter().enumerate() {
        if element - 2 > index as i32 {
            println!("Too chaotic");
            return;
        }
        for e in max(element - 1, 0) as usize..index {
            if queue[e] > element {
                bribes += 1;
            }
        }
    }
    println!("{}", bribes);
}

fn array_manipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut arr = vec![0; (n + 1) as usize];
    let mut maximum = i64::MIN;
    let mut sum = 0i64;

    for query in queries.iter() {
        arr[(query[0] - 1) as usize] += query[2];
        arr[query[1] as usize] -= query[2];
    }

    for &element in arr.iter() {
        sum += element as i64;
        maximum = max(maximum, sum);
    }
    maximum
}

fn check_magazine(magazine: &[String], note: &[String]) {
    let mut map: HashMap<&String, u8> = HashMap::new();
    magazine.iter().for_each(|word| {
        map.entry(word).and_modify(|value| *value += 1).or_insert(1);
    });

    for word in note {
        if let Some(value) = map.get_mut(word) {
            if *value > 0 {
                *value -= 1;
                continue;
            }
        }
        println!("No");
        return;
    }
    print!("Yes");
}

fn two_strings(s1: &str, s2: &str) -> String {
    let set1: HashSet<char> = s1.chars().collect();
    let set2: HashSet<char> = s2.chars().collect();

    for c in set2 {
        if set1.contains(&c) {
            return String::from("YES");
        }
    }

    String::from("NO")
}

fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut char_count = HashMap::new();
    for c in s1.chars() {
        char_count.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }
    for c in s2.chars() {
        if let Some(value) = char_count.get_mut(&c) {
            *value -= 1;
            if *value == 0 {
                char_count.remove(&c);
            }
        } else {
            return false;
        }
    }
    char_count.is_empty()
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    let multiple = a[a.len() - 1] as usize;

    'outer: for i in (0..=b[0]).step_by(multiple).skip(1) {
        for j in 0..max(a.len(), b.len()) {
            if let Some(&x) = a.get(j) {
                if i % x != 0 {
                    continue 'outer;
                }
            }
            if let Some(&y) = b.get(j) {
                if y % i != 0 {
                    continue 'outer;
                }
            }
        }
        count += 1;
    }
    count
}

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut scores_iter = scores.iter().peekable();
    let mut low_score = **scores_iter.peek().unwrap_or(&&0);
    let mut high_score = **scores_iter.peek().unwrap_or(&&0);
    let mut records = vec![0, 0];

    scores_iter.next();

    for &i in scores_iter {
        if i > high_score {
            records[0] += 1;
            high_score = i;
        }
        if i < low_score {
            records[1] += 1;
            low_score = i;
        }
    }
    records
}

fn birthday2(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..=s.len() - m as usize {
        let sum: i32 = s[i..i + m as usize].iter().sum();
        if sum == d {
            count += 1;
        }
    }
    count
}

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    if s.len() < m as usize {
        return 0;
    }

    let mut count = 0;
    let mut current_sum: i32 = s.iter().take(m as usize).sum();

    if current_sum == d {
        count += 1;
    }

    for i in 1..=s.len() - m as usize {
        current_sum = current_sum - s[i - 1] + s[i + m as usize - 1];
        if current_sum == d {
            count += 1;
        }
    }
    count
}

fn divisible_sum_pairs2(k: i32, array: &[i32]) -> i32 {
    let mut pairs = 0;
    for i in 0..array.len() {
        for j in (i + 1)..array.len() {
            if (array[i] + array[j]) % k == 0 {
                pairs += 1;
            }
        }
    }
    pairs
}

fn divisible_sum_pairs(k: i32, array: &[i32]) -> i32 {
    let mut count = 0;
    let mut remainder_counts = HashMap::new();

    for i in 0..array.len() {
        let remainder = array[i] % k;
        let complement = if remainder == 0 { 0 } else { k - remainder };

        count += remainder_counts.get(&complement).unwrap_or(&0);
        *remainder_counts.entry(remainder).or_insert(0) += 1;
    }
    count
}

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut sightings = HashMap::new();
    let mut max_sightings = i32::MIN;
    let mut max_sighted: i32 = 0;

    for &element in arr {
        let count = *sightings.entry(element).and_modify(|e| *e += 1).or_insert(1);
        if count > max_sightings {
            max_sightings = count;
            max_sighted = element;
        } else if count == max_sightings {
            max_sighted = if element < max_sighted {element} else {max_sighted}
        }
    }
    max_sighted
}

fn climbing_leaderboard2(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut rankings = vec![];
    let mut ranked: Vec<&i32> = ranked.iter().collect::<HashSet<&i32>>().into_iter().collect();
    ranked.sort_by(|a,b| b.cmp(a));

    'outer: for &score in player.iter() {
        for position in 0..ranked.len() {
            if score >= *ranked[position] {
                rankings.push((position + 1) as i32);
                continue 'outer;
            }
        }
        rankings.push((ranked.len() + 1) as i32);
    }
    rankings
}

fn climbing_leaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut rankings = vec![];

    // Remove duplicates from ranked and sort in descending order
    let mut ranked: Vec<i32> = ranked.iter().copied().collect::<HashSet<_>>().into_iter().collect();
    ranked.sort_by(|a, b| b.cmp(a));

    // Initialize the pointer for the ranked list
    let mut i = ranked.len();

    // Iterate through player scores in ascending order
    for &score in player.iter() {
        // Move the pointer to find the position for the current player score
        while i > 0 && score >= ranked[i - 1] {
            i -= 1;
        }
        rankings.push((i + 1) as i32);
    }

    rankings
}

fn day_of_programmer(year: i32) -> String {
    match year {
        x if x == 1918 => "26.09.1918".to_string(),
        x if x < 1918 && x % 4 == 0  => format!("12.09.{}", x),
        x if (x % 4 == 0 && x % 100 != 0) || (x % 400 == 0) => format!("12.09.{}", x),
        x => format!("13.09.{}", x),
    }
}

fn icecream_parlor(m: i32, arr: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut result = vec![];

    for (index, &element) in arr.iter().enumerate() {
        if let Some(&first) = map.get(&(m-element)) {
            result.push(first as i32);
            result.push((index + 1) as i32);
            break;
        }
        map.insert(element, index + 1);
    }
    result
}

fn missing_numbers2(arr: &mut [i32], brr: &mut [i32]) -> Vec<i32> {
    let mut  index = 0usize;
    let mut missing = vec![];
    let mut unique_missing = HashSet::new();

    arr.sort();
    brr.sort();

    for i in 0..brr.len() {
        if let Some(&e ) = arr.get(index) {
            if e == brr[i] {
                index += 1;
                continue;
            }
        }

        if !unique_missing.contains(&brr[i]) {
            missing.push(brr[i]);
            unique_missing.insert(&brr[i]);
        }
    }
    missing
}

fn missing_numbers(arr: &[i32], brr: &[i32]) -> Vec<i32> {
    let mut missing = vec![];
    let mut missing_set = HashSet::new();
    let mut map = HashMap::new();

    for &element in arr {
        map.entry(element).and_modify(|e| *e += 1).or_insert(1);
    }

    for &element in brr {
        if let Some(value) = map.get_mut(&element) {
            *value -= 1;
            if *value == 0 {
                map.remove(&element);
            }
        } else {
            if !missing_set.contains(&element) {
                missing_set.insert(element);
                missing.push(element);
            }
        }
    }
    missing.sort();
    missing
}

fn fibonacci_iterative(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let value = a + b;
        a = b;
        b = value;
    }
    if n == 0 {a} else {b}
}

fn pairs2(k: i32, arr: &[i32]) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;

    for &element in arr {
        if element > k {
            map.entry(element - k).and_modify(|e| *e += 1).or_insert(1);
            map.entry(element + k).and_modify(|e| *e += 1).or_insert(1);
        } else if element < k {
            map.entry(element + k).and_modify(|e| *e += 1).or_insert(1);
        }

        if let Some(&value) = map.get(&element) {
            count += value;
        }
    }

    count
}

fn pairs(k: i32, arr: &[i32]) -> i32 {
    let mut set = HashSet::new();
    let mut count = 0;

    for element in arr {
        set.insert(element);

        if set.contains(&(element + k)) {
            count += 1;
        }
        if set.contains(&(element - k)) {
            count += 1;
        }
    }

    count
}

fn balanced_sums2(arr: &[i32]) -> String {
    let arr: Vec<&i32> = arr.into_iter().filter(|&e| *e != 0).collect();
    let mut indexes_left: HashSet<usize> = (0..arr.len()).collect();
    let mut sums = (0, 0);
    let mut current_indexes = (0, arr.len() - 1);

    loop {
        if indexes_left.len() <= 1 {
            break;
        }

        if sums.1 > sums.0 {
            sums.0 += *arr[current_indexes.0];
            indexes_left.remove(&current_indexes.0);
            current_indexes.0 += 1;
        } else if sums.0 > sums.1 {
            sums.1 += *arr[current_indexes.1];
            indexes_left.remove(&current_indexes.1);
            current_indexes.1 -= 1;
        } else {
            sums.0 += *arr[current_indexes.0];
            indexes_left.remove(&current_indexes.0);

            sums.1 += *arr[current_indexes.1];
            indexes_left.remove(&current_indexes.1);

            current_indexes.0 += 1;
            current_indexes.1 -= 1;
        }
    }

    if sums.0 == sums.1 && indexes_left.len() == 1 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn balanced_sums(arr: &[i32]) -> String {
    let mut left = 0;
    let sum: i32 = arr.iter().sum();

    for &element in arr {
        let right = sum - (element + left);
        if left == right {
            return "YES".to_string()
        } else {
            left += element;
        }
    }

    "NO".to_string()
}

fn hackerland_radio_transmitters2(x: &[i32], k: i32) -> i32 {
    let mut x = x.to_vec();
    x.sort();

    let mut transmitter_count = 1;
    let mut previous_element = x[0];
    let mut coverage = -k;
    let mut midpoint = x[0] + k;

    for (i, &element) in x.iter().enumerate() {
        let distance_from_last_index = element - previous_element;
        let new_coverage = coverage + distance_from_last_index;

        if (new_coverage > k) || (coverage.is_negative() && new_coverage.is_positive()) {
            transmitter_count += 1;
            coverage = -k;
            midpoint = element + k;
        } else {
            coverage = new_coverage;
        }

        if element <= midpoint && x.get(i + 1).is_some_and(|e| *e > midpoint) {
            coverage = 0;
        }

        previous_element = element;
    }

    transmitter_count
}

fn hackerland_radio_transmitters(x: &[i32], k: i32) -> i32 {
    let mut x = x.to_vec();
    x.sort();

    let mut transmitter_count = 0;
    let mut i = 0;
    let n = x.len();

    while i < n {  // [2, 4, 5, 6, 9, 12, 15]
        transmitter_count += 1;

        // Place the transmitter at the furthest house within range
        let loc = x[i] + k;
        while i < n && x[i] <= loc {
            i += 1;
        }

        // Move i to the furthest house within transmitter range
        i -= 1;
        let loc = x[i] + k;

        // Skip all houses covered by this transmitter
        while i < n && x[i] <= loc {
            i += 1;
        }
    }

    transmitter_count
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() {
        if nums[i] == val {
            nums[i] = 0;
            count += 1;
        } else {
            nums[i-count] = nums[i];
        }
    }
    (nums.len() - count) as i32
}

fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[count] = nums[i];
            count += 1;
        }
    }
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hackerland_radio_transmitter() {
        // assert_eq!(2, hackerland_radio_transmitters(&[1, 2, 3, 4, 5], 1));
        assert_eq!(4, hackerland_radio_transmitters2(&[9, 5, 4, 2, 6, 15, 12], 2));
        assert_eq!(4, hackerland_radio_transmitters2(&[1, 3, 4, 6, 7, 9, 11, 12, 14, 17], 2));
    } // [2, 4, 5, 6, 9, 12, 15]

    #[test]
    fn test_balanced_sums() {
        assert_eq!("YES".to_string(), balanced_sums(&[1, 2, 3, 3]));
        assert_eq!("YES".to_string(), balanced_sums(&[1]));
        assert_eq!("YES".to_string(), balanced_sums(&[0, 0, 0, 1, 0]));
        assert_eq!("NO".to_string(), balanced_sums(&[1, 2, 3, 4]));

        assert_eq!("YES".to_string(), balanced_sums2(&[1, 2, 3, 3]));
        assert_eq!("NO".to_string(), balanced_sums2(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_pairs() {
        assert_eq!(3, pairs(2, &[1, 5, 3, 4, 2]));
        assert_eq!(3, pairs2(2, &[1, 5, 3, 4, 2]));
    }

    #[test]
    fn test_fibonacci_iterative() {
        assert_eq!(8, fibonacci_iterative(6));
        assert_eq!(5, fibonacci_iterative(5));
        assert_eq!(55, fibonacci_iterative(10));
    }

    #[test]
    fn test_missing_numbers() {
        assert_eq!(vec![2, 5], missing_numbers(&[1, 3, 4], &[1, 2, 3, 4, 5]));
        assert_eq!(vec![2, 5], missing_numbers2(&mut [1, 3, 4], &mut [1, 2, 3, 4, 5]));
        assert_eq!(
            vec![3, 7, 8, 10, 12],
            missing_numbers(
                &[11, 4, 11, 7, 13, 4, 12, 11, 10, 14],
                &[11, 4, 11, 7, 3, 7, 10, 13, 4, 8, 12, 11, 10, 14, 12]
            )
        );
    }

    #[test]
    fn test_icecream_parlor() {
        assert_eq!(vec![1, 4], icecream_parlor(4, &[1, 4, 5, 3, 2]));
    }

    #[test]
    fn test_climbing_leaderboard() {
        assert_eq!(vec![4, 3, 1], climbing_leaderboard(&[100, 90, 90, 80], &[70, 80, 105]));
        assert_eq!(vec![4, 3, 1], climbing_leaderboard2(&[100, 90, 90, 80], &[70, 80, 105]));
    }

    #[test]
    fn test_migratory_birds() {
        assert_eq!(4, migratory_birds(&[1, 4, 4, 4, 5, 3]));
    }

    #[test]
    fn test_divisible_sum_pairs() {
        assert_eq!(5, divisible_sum_pairs(3, &[1, 3, 2, 6, 1, 2]));
        assert_eq!(5, divisible_sum_pairs2(3, &[1, 3, 2, 6, 1, 2]));
    }

    #[test]
    fn test_birthday() {
        assert_eq!(2, birthday(&[2, 2, 1, 3, 2], 4, 2));
        assert_eq!(1, birthday(&[4], 4, 1));
        assert_eq!(0, birthday(&[1, 1, 1, 1, 1], 3, 2));
        assert_eq!(0, birthday2(&[1, 1, 1, 1, 1], 3, 2));
    }

    #[test]
    fn test_breaking_records() {
        assert_eq!(
            vec![2, 4],
            breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1])
        );
        assert_eq!(
            vec![2, 4],
            breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1])
        );
        assert_eq!(
            vec![4, 0],
            breaking_records(&[3, 4, 21, 36, 10, 28, 35, 5, 24, 42])
        );
    }

    #[test]
    fn test_get_total_x() {
        assert_eq!(3, get_total_x(&[2, 4], &[16, 32, 96]));
    }

    #[test]
    fn test_are_anagrams() {
        assert_eq!(true, are_anagrams("conversation", "voicesranton"));
    }

    #[test]
    fn test_two_string() {
        assert_eq!("YES".to_string(), two_strings("hello", "world"));
    }

    #[test]
    fn test_check_magazine() {
        let magazine = ["give", "me", "one", "grand", "today", "night"].map(|e| e.to_string());
        let note = ["give", "one", "grand", "today"].map(|e| e.to_string());

        check_magazine(&magazine[..], &note[..])
    }

    #[test]
    fn test_repeated_string() {
        assert_eq!(repeated_string("aba", 9), 6);
        assert_eq!(repeated_string("aba", 10), 7);
        assert_eq!(repeated_string("aba", 11), 7);
        assert_eq!(repeated_string("aba", 12), 8);
        assert_eq!(repeated_string("abcdaal", 2), 1);
    }

    #[test]
    fn test_rot_left() {
        assert_eq!(rot_left(&[1, 2, 3, 4, 5], 4), &[5, 1, 2, 3, 4]);
    }

    #[test]
    fn test_minimum_bribes() {
        minimum_bribes(&[4, 2, 1, 3]);
        minimum_bribes(&[1, 2, 5, 3, 7, 8, 6, 4]);
    }

    #[test]
    fn test_array_manipulation() {
        let arr1 = &[vec![1, 2, 100], vec![2, 5, 100], vec![3, 4, 100]];
        let arr2 = &[vec![2, 6, 8], vec![3, 5, 7], vec![1, 8, 1], vec![5, 9, 15]];
        assert_eq!(array_manipulation(5, arr1), 200);
        assert_eq!(array_manipulation(10, arr2), 31);
    }
}
