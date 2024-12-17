use ruuf_cuantos_paneles_caben::pallet_loading_problem::cuantos_caben_b_y_d;

fn main() {
    // Si queremos guardar las iteraciones como png o no
    let draw = true;

    // Debiera dar 4
    // dbg!(cuantos_caben_b_y_d(1, 2, 2, 4, draw));

    // // Debiera dar 7
    // dbg!(cuantos_caben_b_y_d(1, 2, 3, 5, draw));

    // // Debiera dar 14
    // dbg!(cuantos_caben_b_y_d(5, 2, 12, 12, draw));

    // // Debiera dar 0
    // dbg!(cuantos_caben_b_y_d(2, 2, 1, 10, draw));
    // // Debiera dar 33
    // dbg!(cuantos_caben_b_y_d(3, 4, 20, 20, draw));

    // dbg!(cuantos_caben_b_y_d(2, 7, 21, 15, draw));
    // dbg!(cuantos_caben_b_y_d(2, 7, 21, 17, draw));
    dbg!(cuantos_caben_b_y_d(2, 5, 29, 17, draw));
}
