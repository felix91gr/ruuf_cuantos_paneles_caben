#![allow(non_snake_case)]
use std::{collections::HashMap, fs::File};

use csv::Reader;
use ruuf_cuantos_paneles_caben::pallet_loading_problem::cuantos_caben_b_y_d;

#[test]
fn tests_del_enunciado() {
    let should_be_4 = cuantos_caben_b_y_d(1, 2, 2, 4);
    let should_be_7 = cuantos_caben_b_y_d(1, 2, 3, 5);
    let should_be_0 = cuantos_caben_b_y_d(2, 2, 1, 10);
    assert_eq!(should_be_4, 4);
    assert_eq!(should_be_7, 7);
    assert_eq!(should_be_0, 0);
}

// Quick helper function to get a reader for the test batteries
fn get_iterator_for_csv_test_file(file_path: &str) -> Reader<File> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(file_path)
        .unwrap()
}

#[test]
fn tests_cover_1_a() {
    run_cover_test("./tests/CoverIA.txt");
}

fn run_cover_test(test_path: &str) {
    type Record = (u32, u32, u32, u32, u32);

    let mut rdr = get_iterator_for_csv_test_file(test_path);

    let mut count_total = 0;
    let mut total_relative_difference: f32 = 0.0;
    let mut differences_vs_optimum: HashMap<u32, u32> = HashMap::new();

    for result in rdr.deserialize::<Record>() {
        if let Ok((L, W, l, w, solution)) = result {
            count_total += 1;

            let our_solution = cuantos_caben_b_y_d(w, l, W, L);

            assert!(our_solution <= solution);

            let delta_vs_optimum = solution - our_solution;

            differences_vs_optimum
                .entry(delta_vs_optimum)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    println!("Resultados de probar con el test {test_path}:");

    let mut deltas: Vec<u32> = differences_vs_optimum.keys().map(|d| *d).collect();
    deltas.sort();

    for delta in deltas {
        let count_of_delta = differences_vs_optimum.get(&delta).unwrap();
        let percentage_of_total = 100.0 * *count_of_delta as f32 / count_total as f32;

        if delta == 0 {
            println!("Encontramos el óptimo para {count_of_delta} de {count_total} escenarios, un {0:.1}% del total", percentage_of_total);
        } else {
            println!("Encontramos una solución {delta} unidades menor al óptimo para {count_of_delta} de {count_total} escenarios, un {0:.2}% del total", percentage_of_total);
        }
    }
}

#[test]
fn tests_cover_1_b() {
    run_cover_test("./tests/CoverIB.txt");
}

#[test]
fn tests_cover_2_a() {
    run_cover_test("./tests/CoverIIA.txt");
}

#[test]
fn tests_cover_2_b() {
    run_cover_test("./tests/CoverIIB.txt");
}

#[test]
fn tests_cover_ships() {
    run_cover_test("./tests/Ships.txt");
}

#[test]
fn tests_cover_3_b() {
    type Record = (u32, u32, u32, u32, u32, u32);

    let mut rdr = get_iterator_for_csv_test_file("./tests/CoverIIIB.txt");

    let mut count_bested = 0;
    let mut count_total = 0;

    for result in rdr.deserialize::<Record>() {
        if let Ok((L, W, l, w, LB, UB)) = result {
            count_total += 1;

            let our_solution = cuantos_caben_b_y_d(w, l, W, L);

            assert!(our_solution <= UB);
            if our_solution >= LB {
                count_bested += 1;
            }
        }
    }

    let percentage_optimal_solutions = (count_bested as f32) / (count_total as f32);

    println!(
        "Found {count_bested} solutions with equal or better lower bounds than the literature out of {count_total}, a {0:.2}% of the total.",
        percentage_optimal_solutions * 100.0
    );
}
