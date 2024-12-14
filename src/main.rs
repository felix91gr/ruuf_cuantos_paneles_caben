use ruuf_cuantos_paneles_caben::pallet_loading_problem::cuantos_caben_b_y_d;

fn main() {
    // Debiera dar 4
    dbg!(cuantos_caben_b_y_d(1, 2, 2, 4));
    // Debiera dar 7
    dbg!(cuantos_caben_b_y_d(1, 2, 3, 5));
    // Debiera dar 0
    dbg!(cuantos_caben_b_y_d(2, 2, 1, 10));
    // Debiera dar 33
    dbg!(cuantos_caben_b_y_d(3, 4, 20, 20));
}
