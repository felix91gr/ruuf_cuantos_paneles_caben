#![allow(non_snake_case)]

pub mod pallet_loading_problem {
    use omage::{colors::*, Components, Config, Image, Rgba};

    const PRINT_PROGRESS_TO_CONSOLE: bool = false;

    /// Calcula cuántos rectángulos de dimensiones (a, b) caben en un rectángulo
    /// de dimensiones (x, y)
    ///
    /// En esta función usamos la heurística de Bischoff y Dowsland
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use ruuf_cuantos_paneles_caben::pallet_loading_problem::cuantos_caben_b_y_d;
    /// let should_be_4 = cuantos_caben_b_y_d(1, 2, 2, 4, false);
    /// let should_be_7 = cuantos_caben_b_y_d(1, 2, 3, 5, false);
    /// let should_be_0 = cuantos_caben_b_y_d(2, 2, 1, 10, false);
    ///
    /// assert_eq!(should_be_4, 4);
    /// assert_eq!(should_be_7, 7);
    /// assert_eq!(should_be_0, 0);
    /// ```
    ///
    /// Versión con enteros por ahora (TODO: versión con floats coming soon?)
    pub fn cuantos_caben_b_y_d(w: u32, l: u32, W: u32, L: u32, draw: bool) -> u32 {
        if w > l {
            // Reordenamos para que w sea menor que l
            cuantos_caben_b_y_d(l, w, W, L, draw)
        } else if W > L {
            // Reordenamos para que W sea menor que L
            cuantos_caben_b_y_d(w, l, L, W, draw)
        } else {
            let P = get_normal_sets(w, l, L);
            let Q = get_normal_sets(w, l, W);

            let mut z_lower_bound = ((L / l) * (W / w)).max((W / l) * (L / w));

            let z_upper_bound = find_upper_bound_continuous(w, l, W, L);

            let mut image_counter = 0;
            let problem_name = format!("./renders/{w}x{l}_into_{W}x{L}");
            if draw {
                let _ = std::fs::create_dir_all(&problem_name);
            }

            if z_lower_bound == z_upper_bound {
                if PRINT_PROGRESS_TO_CONSOLE {
                    println!("El óptimo es un corte de guillotina simple :D");
                }

                let img_path = format!("{problem_name}/guillotine.png");

                if draw {
                    if L % l == 0 {
                        draw_rectangles(L, 0, 0, 0, 0, W, 0, 0, 0, 0, l, w, L, W, &img_path);
                    } else {
                        draw_rectangles(0, L, 0, 0, 0, 0, W, 0, 0, 0, l, w, L, W, &img_path);
                    }
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
                                            let (L3, W3) = (L - L1 - L5, W - W2 - W4);

                                            let z1 = (L1 / l) * (W1 / w);
                                            let z5 = (L5 / l) * (W5 / w);
                                            let z2 = (L2 / w) * (W2 / l);
                                            let z4 = (L4 / w) * (W4 / l);
                                            let z3 = ((L3 / l) * (W3 / w)).max((L3 / w) * (W3 / l));
                                            let z = z1 + z2 + z3 + z4 + z5;

                                            let img_path =
                                                format!("{problem_name}/step_{image_counter}.png");
                                            image_counter += 1;

                                            if draw {
                                                draw_rectangles(
                                                    L1, L2, L3, L4, L5, W1, W2, W3, W4, W5, l, w,
                                                    L, W, &img_path,
                                                );
                                            }

                                            if z > z_lower_bound {
                                                if PRINT_PROGRESS_TO_CONSOLE {
                                                    println!("Encontramos un nuevo máximo: {z}");
                                                }

                                                z_lower_bound = z;
                                                if z_lower_bound == z_upper_bound {
                                                    if PRINT_PROGRESS_TO_CONSOLE {
                                                        println!("Máximo encontrado es óptimo.");
                                                    }

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

                if PRINT_PROGRESS_TO_CONSOLE {
                    println!(
                        "Fin de la búsqueda. No estamos seguros de que el resultado sea óptimo."
                    );
                }

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
        // Estoy dibujando transpuesto, así que en este contexto:
        // A1 y A5 son horizontales
        // A2 y A4 son verticales

        const SCALE: u32 = 50;

        const BLUE: Rgba<u8> = Rgba([60, 80, 255, 240]);
        const ORANGE: Rgba<u8> = Rgba([255, 100, 20, 230]);
        const GREEN: Rgba<u8> = Rgba([100, 255, 60, 180]);
        const BROWN: Rgba<u8> = Rgba([128, 60, 10, 230]);
        const BRICK: Rgba<u8> = Rgba([255, 80, 60, 230]);
        const DEAD_ZONE: Rgba<u8> = Rgba([160, 160, 160, 128]);

        const LIGHT_GRAY: Rgba<u8> = Rgba([160, 160, 160, 255]);

        const OUTPUT_PATH: &str = "output.png";

        let config = Config::new(L * SCALE, W * SCALE, WHITE, None, OUTPUT_PATH, None);

        let mut image = Image::new();

        image.config(config).init().unwrap();

        let mut grid_lines = Vec::with_capacity((L * W) as usize);

        for i in 1..W {
            let line = Components::Line(0, i * SCALE, L * SCALE, i * SCALE, LIGHT_GRAY);
            grid_lines.push(line);
        }
        for i in 1..L {
            let line = Components::Line(i * SCALE, 0, i * SCALE, W * SCALE, LIGHT_GRAY);
            grid_lines.push(line);
        }

        for line in grid_lines.iter() {
            image.add_component(&line);
        }

        let a1 = Components::Rectangle(W1 * SCALE, L1 * SCALE, 0, 0, BLUE);

        image.add_component(&a1);

        let r1 = L1 / l;
        let u1 = W1 / w;

        let mut a1_lines = Vec::with_capacity((r1 * u1) as usize);

        // Hay r1 columnas de ancho l en L1
        for i in 1..r1 {
            let line = Components::Line(i * l * SCALE, 0, i * l * SCALE, W1 * SCALE, BLACK);
            a1_lines.push(line);
        }

        // Y hay u1 filas de largo w en W1
        for i in 1..u1 {
            let line = Components::Line(0, i * w * SCALE, L1 * SCALE, i * w * SCALE, BLACK);
            a1_lines.push(line);
        }

        for line in a1_lines.iter() {
            image.add_component(&line);
        }

        let a2 = Components::Rectangle(W2 * SCALE, L2 * SCALE, (L - L2) * SCALE, 0, ORANGE);

        image.add_component(&a2);

        let s1 = L2 / w;
        let t2 = W2 / l;

        let mut a2_lines = Vec::with_capacity((s1 * t2) as usize);

        // Hay s1 columnas de ancho w en L2
        for i in 1..s1 {
            let line = Components::Line(
                (L - i * w) * SCALE,
                0,
                (L - i * w) * SCALE,
                W2 * SCALE,
                BLACK,
            );
            a2_lines.push(line);
        }

        // Y hay t2 filas de largo l en W2
        for i in 1..t2 {
            let line = Components::Line(
                (L - L2) * SCALE,
                i * l * SCALE,
                L * SCALE,
                i * l * SCALE,
                BLACK,
            );
            a2_lines.push(line);
        }

        for line in a2_lines.iter() {
            image.add_component(&line);
        }

        let a4 = Components::Rectangle(W4 * SCALE, L4 * SCALE, 0, (W - W4) * SCALE, BROWN);
        image.add_component(&a4);

        let s2 = L4 / w;
        let t1 = W4 / l;

        let mut a4_lines = Vec::with_capacity((s2 * t1) as usize);

        // Hay s2 columnas de ancho w en L4
        for i in 1..s2 {
            let line = Components::Line(
                i * w * SCALE,
                (W - W4) * SCALE,
                i * w * SCALE,
                W * SCALE,
                BLACK,
            );
            a4_lines.push(line);
        }

        // Y hay t1 filas de largo l en W4
        for i in 1..t1 {
            let line = Components::Line(
                0,
                (W - i * l) * SCALE,
                L4 * SCALE,
                (W - i * l) * SCALE,
                BLACK,
            );
            a4_lines.push(line);
        }

        for line in a4_lines.iter() {
            image.add_component(&line);
        }

        let a5 = Components::Rectangle(
            W5 * SCALE,
            L5 * SCALE,
            (L - L5) * SCALE,
            (W - W5) * SCALE,
            BRICK,
        );
        image.add_component(&a5);

        let r2 = L5 / l;
        let u2 = W5 / w;

        let mut a5_lines = Vec::with_capacity((s2 * t1) as usize);

        // Hay r2 columnas de ancho l en L5
        for i in 1..r2 {
            let line = Components::Line(
                (L - i * l) * SCALE,
                (W - W5) * SCALE,
                (L - i * l) * SCALE,
                W * SCALE,
                BLACK,
            );
            a5_lines.push(line);
        }

        // Y hay u2 filas de largo w en W5
        for i in 1..u2 {
            let line = Components::Line(
                (L - L5) * SCALE,
                (W - i * w) * SCALE,
                L * SCALE,
                (W - i * w) * SCALE,
                BLACK,
            );
            a5_lines.push(line);
        }

        for line in a5_lines.iter() {
            image.add_component(&line);
        }

        // A3 está posicionado en: (L - L1 - L5, W - W2 - W4)
        // X0: L1
        // Y0: W2
        // XF: L - L5
        // YF: W - W4
        let z3_horizontal = (L3 / l) * (W3 / w);
        let z3_vertical = (L3 / w) * (W3 / l);

        let mut a3_lines = Vec::with_capacity(z3_horizontal.max(z3_vertical) as usize);

        let a3 = if z3_horizontal >= z3_vertical {
            let ancho = L3 / l;
            let alto = W3 / w;
            let rect = Components::Rectangle(
                alto * w * SCALE,
                ancho * l * SCALE,
                L1 * SCALE,
                W2 * SCALE,
                GREEN,
            );

            // Columnas
            for i in 1..ancho {
                let line = Components::Line(
                    (L1 + i * l) * SCALE,
                    W2 * SCALE,
                    (L1 + i * l) * SCALE,
                    (W2 + alto * w) * SCALE,
                    BLACK,
                );
                a3_lines.push(line);
            }

            // Filas
            for i in 1..alto {
                let line = Components::Line(
                    L1 * SCALE,
                    (W2 + i * w) * SCALE,
                    (L1 + ancho * l) * SCALE,
                    (W2 + i * w) * SCALE,
                    BLACK,
                );
                a3_lines.push(line);
            }

            rect
        } else {
            let ancho = L3 / w;
            let alto = W3 / l;
            let rect = Components::Rectangle(
                alto * l * SCALE,
                ancho * w * SCALE,
                L1 * SCALE,
                W2 * SCALE,
                GREEN,
            );

            // Columnas
            for i in 1..ancho {
                let line = Components::Line(
                    (L1 + i * w) * SCALE,
                    W2 * SCALE,
                    (L1 + i * w) * SCALE,
                    (W2 + alto * l) * SCALE,
                    BLACK,
                );
                a3_lines.push(line);
            }

            // Filas
            for i in 1..alto {
                let line = Components::Line(
                    L1 * SCALE,
                    (W2 + i * l) * SCALE,
                    (L1 + ancho * w) * SCALE,
                    (W2 + i * l) * SCALE,
                    BLACK,
                );
                a3_lines.push(line);
            }

            rect
        };

        if z3_horizontal > 0 || z3_vertical > 0 {
            image.add_component(&a3);

            for line in a3_lines.iter() {
                image.add_component(&line);
            }
        }

        // Rectángulos de zonas perdidas
        let ded12 =
            Components::Rectangle(W2 * SCALE, (L - L1 - L2) * SCALE, L1 * SCALE, 0, DEAD_ZONE);
        let ded25 = Components::Rectangle(
            (W - W2 - W5) * SCALE,
            L5 * SCALE,
            (L - L5) * SCALE,
            W2 * SCALE,
            DEAD_ZONE,
        );
        let ded54 = Components::Rectangle(
            W4 * SCALE,
            (L - L4 - L5) * SCALE,
            L4 * SCALE,
            (W - W4) * SCALE,
            DEAD_ZONE,
        );
        let ded41 =
            Components::Rectangle((W - W4 - W1) * SCALE, L1 * SCALE, 0, W1 * SCALE, DEAD_ZONE);

        image.add_components(vec![&ded12, &ded25, &ded54, &ded41]);

        if let Err(e) = image.draw() {
            println!("Error al dibujar imagen: {e}");
        }

        let _ = std::fs::rename(OUTPUT_PATH, path);
    }
}
