#![allow(non_snake_case)]

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

/// Calcula cuántos rectángulos de dimensiones (a, b) caben en un rectángulo
/// de dimensiones (x, y)
///
/// En esta función usamos la heurística de Bischoff y Dowsland
///
/// # Ejemplos
///
/// ```
/// let should_be_4 = cuantos_caben_b_y_d(1, 2, 2, 4);
/// let should_be_7 = cuantos_caben_b_y_d(1, 2, 3, 5);
/// let should_be_0 = cuantos_caben_b_y_d(2, 2, 1, 10);
///
/// assert_eq!(should_be_4, 4);
/// assert_eq!(should_be_7, 7);
/// assert_eq!(should_be_0, 0);
/// ```
///
/// Versión con enteros por ahora (TODO: versión con floats coming soon?)
fn cuantos_caben_b_y_d(w: u32, l: u32, W: u32, L: u32) -> u32 {
    if w > l {
        // Reordenamos para que w sea menor que l
        cuantos_caben_b_y_d(l, w, W, L)
    } else if W > L {
        // Reordenamos para que W sea menor que L
        cuantos_caben_b_y_d(w, l, L, W)
    } else {
        let P = get_normal_sets(w, l, L);
        let Q = get_normal_sets(w, l, W);

        let mut z_lower_bound = ((L / l) * (W / w)).max((W / l) * (L / w));

        let L_star = w + P.iter().map(|(r, s)| r * l + s * w).max().unwrap();
        let W_star = w + Q.iter().map(|(t, u)| t * l + u * w).max().unwrap();

        let z_upper_bound = (L_star * W_star) / (l * w);

        dbg!(z_lower_bound);
        dbg!(z_upper_bound);

        if z_lower_bound == z_upper_bound {
            dbg!("Corte de guillotina simple");
            z_lower_bound
        } else {
            for (r1, s1) in P.iter() {
                for (r2, s2) in P.iter() {
                    if r1 * l + s1 * w <= r2 * l + s2 * w {
                        for (t1, u1) in Q.iter() {
                            for (t2, u2) in Q.iter() {
                                if t1 * l + u1 * w <= t2 * l + u2 * w {
                                    let (L1, W1) = (r1 * l, u1 * w);
                                    let (L2, W2) = (s1 * w, t2 * l);
                                    let (L4, W4) = (s2 * w, t1 * l);
                                    let (L5, W5) = (r2 * l, u2 * w);

                                    let overlap_1 = (L1 + L5 > L) && (W1 + W5 > W);
                                    let overlap_2 = (L2 + L4 > L) && (W2 + W4 > W);
                                    if !overlap_1 && !overlap_2 {
                                        // We use abs_diff because we avoid negative numbers
                                        let (L3, W3) = (L.abs_diff(L1 + L5), W.abs_diff(W2 + W4));

                                        let z1 = (L1 / l) * (W1 / w);
                                        let z5 = (L5 / l) * (W5 / w);
                                        let z2 = (L2 / w) * (W2 / l);
                                        let z4 = (L4 / w) * (W4 / l);
                                        let z3 = ((L3 / l) * (W3 / w)).max((L3 / w) * (W3 / l));
                                        let z = z1 + z2 + z3 + z4 + z5;
                                        if z > z_lower_bound {
                                            println!("------------------");

                                            println!("New maximum found: {z}");

                                            z_lower_bound = z;
                                            if z_lower_bound == z_upper_bound {
                                                println!("Optimal has been found.");
                                                return z;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            println!("Finished. Unsure if optimal");

            z_lower_bound
        }
    }
}
/// Crea los "sets normales" que cubren una longitud L con segmentos
/// enteros de largo w o l
fn get_normal_sets(w: u32, l: u32, L: u32) -> Vec<(u32, u32)> {
    let mut r = 1 + L / l;
    let mut s;

    let mut normal_set = Vec::with_capacity(r as usize);

    while r > 0 {
        r -= 1;
        s = (L - r * l) / w;
        normal_set.push((r, s));
    }

    normal_set
}
