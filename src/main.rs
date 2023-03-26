use std::collections::{HashSet, HashMap};
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
    pancakes[..index+1].reverse();
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

fn iddfs(permutacion_inicial: &[char]) -> Vec<char> {
    let n = permutacion_inicial.len();
    let mut permutacion_ordenada = permutacion_inicial.to_vec();
    permutacion_ordenada.sort();
    let mut depth_limit = 1; // profundidad máxima inicial
    loop {
        let mut visitados = HashSet::new();
        visitados.insert(hash_permutation(permutacion_inicial));
        let mut stack = Vec::new();
        stack.push((permutacion_inicial.to_owned(), 0, 0));
        let mut d = HashMap::<String, usize>::new();
        let mut p = HashMap::<String, Vec<char>>::new();

        let initial_permutation_hash = hash_permutation(permutacion_inicial);
        d.insert(initial_permutation_hash.clone(), 0);
        p.insert(initial_permutation_hash.clone(), permutacion_inicial.to_vec());

        while let Some((permutacion, index, prev_level)) = stack.pop() {
            let mut level: i32 = 0;
            level = prev_level + 1;

            if is_pancake_sorted(&permutacion) {
                // si se encuentra la permutación ordenada, se detiene la búsqueda
                println!("NÚMERO DE NODOS EXPANDIDOS: {}", visitados.len());
                println!("NIVEL: {}", level);
                println!("NÚMERO DE NODOS EN LA PILA: {}", stack.len());
                println!("INDICE: {}", index);
                return permutacion;
            }

            if level >= depth_limit {
                continue; // se detiene la búsqueda si se alcanza la profundidad máxima actual
            }

            for i in 2..=n {
                let mut sucesor = permutacion.clone();
                flip_pancakes(&mut sucesor, i - 1);
                let sucesor_hash = hash_permutation(&sucesor);
                if !visitados.contains(&sucesor_hash) {
                    visitados.insert(sucesor_hash.clone());
                    d.insert(sucesor_hash.clone(), d[&hash_permutation(&permutacion)] + 1);
                    p.insert(sucesor_hash.clone(), permutacion.clone());
                    stack.push((sucesor, i - 1, level));
                }
            }
        }

        depth_limit += 1; // aumenta la profundidad máxima para la siguiente iteración
    }
}


fn main() {
    let mut n = String::new();
    println!("Ingrese el numero de caracteres de pancakes: ");
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let pancakes = fill_pancakes(n);
    println!("Pancakes generados: {:?}", pancakes);
    iddfs(&pancakes);
}