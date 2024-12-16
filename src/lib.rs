#![allow(non_snake_case)]

pub mod pallet_loading_problem {
    use omage::{colors::*, Components, Config, Image, Rgba};

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

            let mut image_counter = 0;
            let problem_name = format!("./renders/{w}x{l}_into_{W}x{L}");
            std::fs::create_dir_all(&problem_name);

            if z_lower_bound == z_upper_bound {
                println!("El óptimo es un corte de guillotina simple :D");
                let img_path = format!("{problem_name}/guillotine.png");

                if L % l == 0 {
                    draw_rectangles(
                        L, 0, 0, 0, 0, W, 0, 0, 0, 0, l,
                        w, L, W, &img_path
                    );    
                } else {
                    draw_rectangles(
                        0, L, 0, 0, 0, 0, W, 0, 0, 0, l,
                        w, L, W, &img_path
                    );
                }

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
                                            
                                            let img_path = format!("{problem_name}/step_{image_counter}.png");
                                            image_counter += 1;

                                            // TODO: this is a hack.
                                            // I've inverted L2 <-> L4 and W2 <-> W4 because
                                            // I've probably done their rendering wrong.
                                            // I've got to figure out why they got inverted, and
                                            // fix it.
                                            draw_rectangles(
                                                L1, L4, L3, L2, L5, W1, W4, W3, W2, W5, l,
                                                w, L, W, &img_path
                                            );

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

    fn draw_rectangles(
        L1: u32,
        L2: u32,
        L3: u32,
        L4: u32,
        L5: u32,
        W1: u32,
        W2: u32,
        W3: u32,
        W4: u32,
        W5: u32,
        l: u32,
        w: u32,
        L: u32,
        W: u32,
        path: &str,
    ) {

        const SCALE: u32 = 50;

        const BLUE: Rgba<u8> = Rgba([60, 80, 255, 255]);
        const ORANGE: Rgba<u8> = Rgba([255, 100, 20, 255]);
        const GREEN: Rgba<u8> = Rgba([60, 255, 60, 255]);
        const BROWN: Rgba<u8> = Rgba([128, 60, 10, 255]);

        const OUTPUT_PATH : &str =  "output.png";

        let config = Config::new(W * SCALE, L * SCALE, WHITE, None,OUTPUT_PATH, None);

        let mut image = Image::new();

        image.config(config).init().unwrap();

        let a1 = Components::Rectangle(L1 * SCALE, W1 * SCALE, 0, 0, BLUE);

        image.add_component(&a1);

        let r1 = L1 / l;
        let u1 = W1 / w;

        let mut a1_lines = Vec::with_capacity((r1 * u1) as usize);

        // Hay r1 filas de largo l en L1
        for i in 1..r1 {
            let line = Components::Line(0, i * l * SCALE, W1 * SCALE, i * l * SCALE, BLACK);
            a1_lines.push(line);
        }

        // Y hay u1 columnas de ancho w en W1
        for i in 1..u1 {
            let line = Components::Line(i * w * SCALE, 0, i * w * SCALE, L1 * SCALE, BLACK);
            a1_lines.push(line);
        }

        for line in a1_lines.iter() {
            image.add_component(&line);
        }

        let a2 = Components::Rectangle(L2 * SCALE, W2 * SCALE, (W - W2) * SCALE, 0, ORANGE);
        
        image.add_component(&a2);
        
        let s1 = L2 / w;
        let t2 = W2 / l;

        let mut a2_lines = Vec::with_capacity((s1 * t2) as usize);

        // Hay s1 filas de largo w en L2
        for i in 1..s1 {
            let line = Components::Line((W - W2) * SCALE, i * w * SCALE, W * SCALE, i * w * SCALE, BLACK);
            a2_lines.push(line);
        }

        // Y hay t2 columnas de ancho l en W2
        for i in 1..t2 {
            let line = Components::Line((W - i * l) * SCALE, 0, (W - i * l) * SCALE, L2 * SCALE, BLACK);
            a2_lines.push(line);
        }

        for line in a2_lines.iter() {
            image.add_component(&line);
        }
        
        
        let a4 = Components::Rectangle(L4 * SCALE, W4 * SCALE, 0, (L - L4) * SCALE, BROWN);
        image.add_component(&a4);
        
        let s2 = L4 / w;
        let t1 = W4 / l;

        let mut a4_lines = Vec::with_capacity((s2 * t1) as usize);

        // Hay s2 filas de largo w en L4
        for i in 1..s2 {
            let line = Components::Line(0, (L - i * w) * SCALE, W4 * SCALE, (L - i * w) * SCALE, BLACK);
            a4_lines.push(line);
        }

        // Y hay t1 columnas de ancho l en W4
        for i in 1..t1 {
            let line = Components::Line(i * l * SCALE, (L - L4) * SCALE, i * l * SCALE, L * SCALE, BLACK);
            a4_lines.push(line);
        }

        for line in a4_lines.iter() {
            image.add_component(&line);
        }


        let a5 = Components::Rectangle(L5 * SCALE, W5 * SCALE, (W - W5) * SCALE, (L - L5) * SCALE, RED);
        image.add_component(&a5);

        let r2 = L5 / l;
        let u2 = W5 / w;

        let mut a5_lines = Vec::with_capacity((s2 * t1) as usize);

        // Hay r2 filas de largo l en L5
        for i in 1..r2 {
            let line = Components::Line((W - W5) * SCALE, (L - i * l) * SCALE, W * SCALE, (L - i * l) * SCALE, BLACK);
            a5_lines.push(line);
        }

        // Y hay u2 columnas de ancho w en W5
        for i in 1..u2 {
            let line = Components::Line((W - i * w) * SCALE, (L - L5) * SCALE, (W - i * w) * SCALE, L * SCALE, BLACK);
            a5_lines.push(line);
        }

        for line in a5_lines.iter() {
            image.add_component(&line);
        }


        image.draw();

        std::fs::rename(OUTPUT_PATH, path);
    }
}
