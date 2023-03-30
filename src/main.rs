use std::collections::{HashSet};
use rand::{seq::IteratorRandom};
use md5::{Md5, Digest};

fn fill_pancakes(num_pancakes: usize) -> Vec<char> {
    let dict = "abcdefghijklmnopqrstuvwxyz";
    let mut empty_pancakes = Vec::new();
    let mut seen_chars = HashSet::new();
    for _ in 0..num_pancakes {
        let mut random_char = dict.chars().choose(&mut rand::thread_rng()).unwrap();
        while seen_chars.contains(&random_char) {
            random_char = dict.chars().choose(&mut rand::thread_rng()).unwrap();
        }
        seen_chars.insert(random_char);
        empty_pancakes.push(random_char);
    }
    empty_pancakes
}

fn flip_pancakes(pancakes: &mut [char], index: usize) {
    if pancakes.len() < 2 { return; }
    pancakes[..index].reverse();
}

fn is_pancake_sorted(pancakes: &[char]) -> bool {
    for i in 1..pancakes.len() {
        if pancakes[i] < pancakes[i - 1] {
            return false;
        }
    }
    true
}

fn hash_permutation(permutation: &[char]) -> String {
    let input_string: String = permutation.iter().collect();
    let mut hasher = Md5::new();
    hasher.update(input_string.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn iddfs_rec(pancakes: &mut [char], depth: usize, mut visited: &mut HashSet<String>) -> Option<Vec<char>> {
    if depth == 0 {
        return None;
    }
    if is_pancake_sorted(pancakes) {
        return Some(pancakes.to_vec());
    }
    let n = pancakes.len();
    let min_depth = depth;
    let mut result = None;
    for i in 2..=n {
        flip_pancakes(pancakes, i);
        let hash = hash_permutation(pancakes);
        if let Some(_) = result {
            if let Some(r) = iddfs_rec(pancakes, min_depth - 1, &mut visited) {
                result = Some(r);
            }
        } else {
            if !visited.contains(&hash) {
                visited.insert(hash.clone());
                if let Some(r) = iddfs_rec(pancakes, depth - 1, &mut visited) {
                    result = Some(r);
                }
            }
        }
        flip_pancakes(pancakes, i);
    }
    result
}

fn iddfs(pancakes: &mut [char]) -> Option<Vec<char>> {
    let mut depth = 1;
    loop {
        let mut visited = HashSet::new();
        visited.insert(hash_permutation(pancakes));
        if let Some(result) = iddfs_rec(pancakes, depth, &mut visited) {
            println!("Nodos Visitados: {:?}", visited.len());
            return Some(result);
        }
        depth += 1;
    }
}



fn main() {
    let mut n = String::new();
    println!("Ingrese el numero de caracteres de pancakes: ");
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let mut pancakes = fill_pancakes(n);
    println!("Pancakes generados: {:?}", pancakes);
    if let Some(result) = iddfs(&mut pancakes) {
        println!("Pancakes despues de ordenar: {:?}", result);
    } else {
        println!("No se encontro una solución!");
    }
}