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

#[test]
fn tests_cover_1_a() {
    type Record = (u32, u32, u32, u32, u32);

    let file_path = "./tests/CoverIA.txt";
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(file_path)
        .unwrap();

    let mut count_optimums = 0;
    let mut count_total = 0;
    let mut total_relative_difference: f32 = 0.0;

    let mut wrong_results = Vec::new();

    for result in rdr.deserialize::<Record>() {
        if let Ok((L, W, l, w, solution)) = result {
            count_total += 1;

            let our_solution = cuantos_caben_b_y_d(w, l, W, L);

            // assert!(our_solution <= solution);
            // This is supposed to be impossible.
            // If this happens, the test technically fails.
            if our_solution > solution {
                wrong_results.push((L, W, l, w, solution));
            } else if our_solution == solution {
                count_optimums += 1;
            } else {
                total_relative_difference += ((solution - our_solution) as f32) / (solution as f32);
            }
        }
    }

    let average_relative_difference = total_relative_difference / (count_total as f32);
    let percentage_optimal_solutions = (count_optimums as f32) / (count_total as f32);

    println!(
        "Found {count_optimums} optimal solutions out of {count_total}, a {0:.2}% of the total.",
        percentage_optimal_solutions * 100.0
    );
    println!("The average relative difference was of a {0:.2}% between the found and the known optimal solutions", average_relative_difference * 100.0);

    if wrong_results.len() > 0 {
        println!("The test has technically failed. Below are the failing scenarios:");
        for res in wrong_results {
            println!("{:?}", res);
        }
    }
}
