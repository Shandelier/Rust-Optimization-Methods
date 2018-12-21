mod file_reader;
mod print_utils;
mod graph_generator;
mod tabu_search;
mod simulated_annealing;

use std::io;
use std::thread;

fn main() {
    println!();
    println!("Projektowanie efektywnych algorytmów - projekt 2");
    println!("Problem komiwojażera (TSP)");

    // Zmienna przechowująca graf w postaci macierzowej
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    // Kryterium stopu: maksymalny czas wykonania
    let mut time_max: i64 = 60;

    // [SA] Współczynnik zmiany temperatury
    let mut sa_annealing_velocity: f32 = 0.999f32;

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
        println!("5. Symulowane wyżarzanie");
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

                matrix = file_reader::read_any_file(String::from(file_name.trim()), true);

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

                - [TS] Kryterium sąsiedztwa
                - [TS] Iteracje
                - [TS] Kadencja
                - [TS] Błędy
                */

                let mut input = String::new();

                println!("Maksymalny czas wykonania algorytmu [s] [teraz: {}]: ", time_max);
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                if !input.trim().is_empty() {
                    time_max = input.trim().parse().expect("Błędna wartość");
                }
                input.clear();


                println!("[SA] Temperatura początkowa [teraz: {}]: ", sa_temperature);
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                if !input.trim().is_empty() {
                    sa_temperature = input.trim().parse().expect("Błędna wartość");
                }
                input.clear();


                println!("[SA] Współczynnik zmiany temperatury [teraz: {}]: ", sa_annealing_velocity);
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                if !input.trim().is_empty() {
                    sa_annealing_velocity = input.trim().parse().expect("Błędna wartość");
                }
                input.clear();

                // TODO: Uncomment when implemented.
                // println!("[TS] Definicja sąsiedztwa [0 - swap, 1 - insert, 2 - inverse] [teraz: {}]: ", ts_neighbourhood_definition);
                // io::stdin().read_line(&mut input).expect(
                //    "Błąd wejścia/wyjścia",
                // );
                // if !input.trim().is_empty() {
                //    ts_neighbourhood_definition = input.trim().parse().expect("Błędna wartość");
                // }
                // input.clear();


                println!("[TS] Iteracje  [teraz: {}]: ", ts_iterations);
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                if !input.trim().is_empty() {
                    ts_iterations = input.trim().parse().expect("Błędna wartość");
                }
                input.clear();


                println!("[TS] Kadencja blokowanej ścieżki [teraz: {}]: ", ts_lifetime);
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                if !input.trim().is_empty() {
                    ts_lifetime = input.trim().parse().expect("Błędna wartość");
                }
                input.clear();


                println!("[TS] Ilość błędów do uznania za krytyczny [teraz: {}]: ", ts_critical_events);
                io::stdin().read_line(&mut input).expect(
                    "Błąd wejścia/wyjścia",
                );
                if !input.trim().is_empty() {
                    ts_critical_events = input.trim().parse().expect("Błędna wartość");
                }
                input.clear();
            }
            4 => {
                if matrix.is_empty() {
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    tabu_search::solve(&mut matrix,
                                       ts_iterations,
                                       ts_lifetime,
                                       ts_critical_events,
                                       time_max,
                                       ts_neighbourhood_definition,
                                       true);
                }
            }
            444 => {
                // (nazwa pliku, czas maksymalny)
                let datafiles = vec![("ftv47.atsp", 120), ("ftv170.atsp", 240), ("rbg403.atsp", 360)];
                for data in datafiles {
                    matrix = file_reader::read_any_file("data/".to_owned() + data.0, false);

                    let mut children = vec![];

                    for _ in 0..10 {
                        let mut x = matrix.clone();
                        let time = data.1;
                        // (iteracje, kadencja, błędy)
                        // iteracje: 5k, 10k, 20k
                        // kadencja: 10, 20, 40
                        // błędy: 10, 25, 75
                        let versions = vec![(5000, 10, 10), (5000, 10, 25), (5000, 10, 75),
                                            (5000, 20, 10), (5000, 20, 25), (5000, 20, 75),
                                            (5000, 40, 10), (5000, 40, 25), (5000, 40, 75),
                                            (10000, 10, 10), (10000, 10, 25), (10000, 10, 75),
                                            (10000, 20, 10), (10000, 20, 25), (10000, 20, 75),
                                            (10000, 40, 10), (10000, 40, 25), (10000, 40, 75),
                                            (20000, 10, 10), (20000, 10, 25), (20000, 10, 75),
                                            (20000, 20, 10), (20000, 20, 25), (20000, 20, 75),
                                            (20000, 40, 10), (20000, 40, 25), (20000, 40, 75)];
                        children.push(thread::spawn(move || {
                            for version in versions {
                                tabu_search::solve(&mut x, 
                                               version.0, 
                                               version.1, 
                                               version.2, 
                                               time, 
                                               0, 
                                               false);
                            }
                        }));
                    }
                
                    for child in children {
                        let _ = child.join();
                    }
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
            555 => {
                // (nazwa pliku, czas maksymalny)
                let datafiles = vec![("ftv47.atsp", 60), ("ftv170.atsp", 120), ("rbg403.atsp", 180)];

                for data in datafiles {
                    matrix = file_reader::read_any_file("data/".to_owned() + data.0, false);
                    let mut children = vec![];

                    for _ in 0..10 {
                        let mut x = matrix.clone();
                        let timeout = data.1;
                        children.push(thread::spawn(move || {
                            simulated_annealing::solve(&mut x, 10f32, 0.99999, timeout);
                            simulated_annealing::solve(&mut x, 20f32, 0.99999, timeout);
                            simulated_annealing::solve(&mut x, 40f32, 0.99999, timeout);
                            simulated_annealing::solve(&mut x, 10f32, 0.999994, timeout);
                            simulated_annealing::solve(&mut x, 20f32, 0.999994, timeout);
                            simulated_annealing::solve(&mut x, 40f32, 0.999994, timeout);
                            simulated_annealing::solve(&mut x, 10f32, 0.999999, timeout);
                            simulated_annealing::solve(&mut x, 20f32, 0.999999, timeout);
                            simulated_annealing::solve(&mut x, 40f32, 0.999999, timeout);
                        }));
                    }

                    for child in children {
                        let _ = child.join();
                    }
                }
            }
            _ => println!("Niepoprawna wartość!"),
        }
    };
}
