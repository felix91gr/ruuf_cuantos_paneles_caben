#![allow(non_snake_case)]

pub mod pallet_loading_problem {

    /// Calcula cuántos rectángulos de dimensiones (a, b) caben en un rectángulo
    /// de dimensiones (x, y)
    ///
    /// En esta función usamos la heurística de Bischoff y Dowsland
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use ruuf_cuantos_paneles_caben::pallet_loading_problem::cuantos_caben_b_y_d;
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
    pub fn cuantos_caben_b_y_d(w: u32, l: u32, W: u32, L: u32) -> u32 {
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

            let z_upper_bound = find_upper_bound_continuous(w, l, W, L);

            if z_lower_bound == z_upper_bound {
                println!("El óptimo es un corte de guillotina simple :D");
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

                                        let Z3_is_positive = L > L1 + L5 && W > W2 + W4;
                                        if Z3_is_positive && !overlap_1 && !overlap_2 {
                                            // We use abs_diff because we avoid negative numbers
                                            let (L3, W3) = (L - L1 - L5, W - W2 - W4);

                                            let z1 = (L1 / l) * (W1 / w);
                                            let z5 = (L5 / l) * (W5 / w);
                                            let z2 = (L2 / w) * (W2 / l);
                                            let z4 = (L4 / w) * (W4 / l);
                                            let z3 = ((L3 / l) * (W3 / w)).max((L3 / w) * (W3 / l));
                                            let z = z1 + z2 + z3 + z4 + z5;

                                            if z > z_lower_bound {
                                                println!("Encontramos un nuevo máximo: {z}");

                                                z_lower_bound = z;
                                                if z_lower_bound == z_upper_bound {
                                                    println!("Máximo encontrado es óptimo.");
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

                println!("Fin de la búsqueda. No estamos seguros de que el resultado sea óptimo.");

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

    /// Calcula una cota superior a la cantidad de paneles w x l que caben en W x L,
    /// tomando la parte entera del cociente entre el área del rectángulo grande y
    /// el área del rectángulo pequeño.
    fn find_upper_bound_continuous(w: u32, l: u32, W: u32, L: u32) -> u32 {
        let area_small_rectangle = w * l;
        let area_large_rectangle = W * L;
        area_large_rectangle / area_small_rectangle
    }
}
