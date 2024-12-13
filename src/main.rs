fn main() {
    // Esto funciona bien!
    // dbg!(get_normal_sets(3, 4, 20));

    dbg!(cuantos_caben_b_y_d(1, 2, 2, 4));
}

/// Calcula cuántos rectángulos de dimensiones (a, b) caben en un rectángulo
/// de dimensiones (x, y)
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
        dbg!(&l);
        dbg!(&w);
        dbg!(&L);
        dbg!(&W);

        let P = get_normal_sets(w, l, L);

        let Q = get_normal_sets(w, l, W);

        dbg!(&P);
        dbg!(&Q);

        let mut z_lower_bound = ((L / l) * (W / w)).max((W / l) * (L / w));

        let L_star = w + P.iter().map(|(r, s)| r * l + s * w).max().unwrap();
        let W_star = w + Q.iter().map(|(t, u)| t * l + u * w).max().unwrap();

        dbg!(&L_star);
        dbg!(&W_star);

        let z_upper_bound = (L_star * W_star) / (l * w);

        dbg!(z_lower_bound);
        dbg!(z_upper_bound);

        if z_lower_bound == z_upper_bound {
            dbg!("Corte de guillotina simple");
            z_lower_bound
        } else {
            unimplemented!()
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
