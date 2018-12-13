mod file_reader;
mod print_utils;
mod graph_generator;
mod tabu_search;
mod simulated_annealing;

use std::io;

fn main() {
    println!();
    println!("Projektowanie efektywnych algorytmów - projekt 2");
    println!("Problem komiwojażera (TSP)");

    // Zmienna przechowująca graf w postaci macierzowej
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    // Kryterium stopu: maksymalny czas wykonania
    let mut time_max: i64 = 180;

    // [SA] Współczynnik zmiany temperatury
    let mut sa_annealing_velocity: f32 = 0.0001f32;

    // [SA] Temperatura
    let mut sa_temperature: f32 = 10000.0f32;

    // [TS] Kryterium sąsiedztwa
    // 0 - swap
    // TODO: 1 - insert
    // TODO: 2 - inverse
    let mut ts_neighbourhood_definition: i32 = 0;

    // [TS] Iteracje
    let mut ts_iterations: i32 = 10000;

    // [TS] Kadencje
    let mut ts_lifetime: i32 = 20;

    // [TS] Błędy
    let mut ts_critical_events: i32 = 75;


    loop {
        let mut selected_number = String::new();

        println!();
        println!("Wybierz funkcję:");
        println!("1. Wczytaj graf z pliku");
        println!("2. Generuj graf");
        println!("3. Ustaw parametry");
        println!("4. Tabu Search");
        println!("5. Symulowane Wyżarzanie");
        println!("0. Wyjście");
        println!("Wybór: ");

        //Wczytaj odpowiedź użytkownika
        io::stdin().read_line(&mut selected_number).expect(
            "Błąd wejścia/wyjścia",
        );

        //Zmień typ odpowiedzi na integer
        let selected_number: u32 = selected_number.trim().parse().expect("Błędna wartość");

        //Podejmij akcję w zalezności od wyboru użytkownika
        match selected_number {
            0 => std::process::exit(0),
            1 => {
                let mut file_name = String::new();
                println!("Nazwa pliku: ");
                io::stdin()
                    .read_line(&mut file_name)
                    .expect(
                    "Błąd wejścia/wyjścia",
                );

                matrix = file_reader::read_any_file(String::from(file_name.trim()));

                print_utils::print_matrix(&matrix);
            }
            2 => {
                let mut number_of_nodes: String = String::new();
                println!("Ilość wierzchołków:");
                io::stdin()
                    .read_line(&mut number_of_nodes)
                    .expect(
                        "Błąd wejścia/wyjścia",
                    );

                let number_of_nodes: i32 = number_of_nodes.trim()
                    .parse()
                    .expect("Błędna wartość");

                matrix = graph_generator::generate_random_graph(number_of_nodes);
                print_utils::print_matrix(&matrix);
            }
            3 => {
                /*
                - Kryterium stopu: czas
                - [SA] Współczynnik zmiany temperatury
                - [SA] Temperatura
                -
                - [TS] Kryterium sąsiedztwa
                - [TS] Iteracje
                - [TS]
                */

                let mut input = String::new();

                println!("Maksymalny czas wykonania algorytmu [s]: ");
                //Wczytaj odpowiedź użytkownika
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );

                let max_time: i64 = input.trim().parse().expect("Błędna wartość");
                input.clear();

                println!("[SA] Temperatura początkowa: ");
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );

                let sa_temp: f32 = input.trim().parse().expect("Błędna wartość");
                input.clear();

                println!("[SA] Współczynnik zmiany temperatury: ");
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );

                let sa_ann_vel: f32 = input.trim().parse().expect("Błędna wartość");
                input.clear();

// TODO: Uncomment after implementation.
//                println!("[TS] Definicja sąsiedztwa [0, 1, 2]: ");
//                io::stdin().read_line(&mut input).expect(
//                    "Błąd wejścia/wyjścia",
//                );
//                let ts_neigh_def: i32 = input.trim().parse().expect("Błędna wartość");
                let ts_neigh_def: i32 = 0;
//                input.clear();
//                if !(ts_neigh_def == 0 || ts_neigh_def == 1 || ts_neigh_def == 2) {
//                    panic!("Błędna wartość");
//                }

                println!("[TS] Iteracje: ");
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                let ts_iter: i32 = input.trim().parse().expect("Błędna wartość");
                input.clear();

                println!("[TS] Kadencja blokowanej ścieżki: ");
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                let ts_lt: i32 = input.trim().parse().expect("Błędna wartość");
                input.clear();

                println!("[TS] Ilość błędów do uznania za krytyczny: ");
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                let ts_err: i32 = input.trim().parse().expect("Błędna wartość");
                input.clear();

                time_max = max_time;
                sa_temperature = sa_temp;
                sa_annealing_velocity = sa_ann_vel;
                ts_neighbourhood_definition = ts_neigh_def;
                ts_iterations = ts_iter;
                ts_lifetime = ts_lt;
                ts_critical_events = ts_err;
            }
            4 => {
                if matrix.is_empty() {
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    tabu_search::solve(&mut matrix,
                                       ts_iterations,
                                       ts_lifetime,
                                       ts_critical_events,
                                       time_max);
                }
            }
            5 => {
                if matrix.is_empty() {
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    simulated_annealing::solve(&mut matrix,
                                               sa_temperature,
                                               sa_annealing_velocity,
                                               time_max);
                }
            }
            _ => println!("Niepoprawna wartość!"),
        }
    };
}
