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

    //Zmienna przechowująca graf w postaci macierzowej
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    loop {
        let mut selected_number = String::new();

        println!();
        println!("Wybierz funkcję:");
        println!("1. Wczytaj plik z grafem");
        println!("2. Wygeneruj losowy graf");
        println!("3. Przeszukiwanie tabu");
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
            0 => std::process::exit(0x0),
            1 => {
                let mut file_name = String::new();
                println!("Nazwa pliku: ");
                io::stdin().read_line(&mut file_name).expect(
                    "Błąd wejścia/wyjścia",
                );

                matrix = file_reader::read_any_file(String::from(file_name.trim()));

                print_utils::print_matrix(&matrix);
            }
            2 => {
                let mut number_of_nodes: String = String::new();
                println!("Ilość wierzchołków:");
                io::stdin().read_line(&mut number_of_nodes).expect(
                    "Błąd wejścia/wyjścia",
                );

                let number_of_nodes: i32 = number_of_nodes.trim().parse().expect("Błędna wartość");

                matrix = graph_generator::generate_random_graph(number_of_nodes);
                print_utils::print_matrix(&matrix);
            }
            3 => {
                if matrix.is_empty() {
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    solve_tsp(&mut matrix)
                }
            }
            4 => {
                if matrix.is_empty() {
                    println!("Najpierw wczytaj graf z pliku!");
                } else {
                    simulated_annealing::prepare(&mut matrix);
                }
            }
            _ => println!("Niepoprawna wartość!"),
        }
    };

    fn solve_tsp(mut matrix: &mut Vec<Vec<i32>>) {

        // Wczytanie iteracji
        println!("Iteracje:");
        let mut iterations: String = String::new();
        io::stdin().read_line(&mut iterations).expect(
            "Błąd wejścia/wyjścia",
        );
        let iterations: i32 = iterations.trim().parse().expect("Błędna wartość");

        // Wczytanie kadencji
        println!("Kadencje:");
        let mut lifetime: String = String::new();
        io::stdin().read_line(&mut lifetime).expect(
            "Błąd wejścia/wyjścia",
        );
        let lifetime: i32 = lifetime.trim().parse().expect("Błędna wartość");

        // Maksymalna liczba błędów
        println!("Błędy:");
        let mut critical_events: String = String::new();
        io::stdin().read_line(&mut critical_events).expect(
            "Błąd wejścia/wyjścia",
        );
        let critical_events: i32 = critical_events.trim().parse().expect("Błędna wartość");

        // Maksymalny czas
        println!("Maksymalny czas:");
        let mut max_time: String = String::new();
        io::stdin().read_line(&mut max_time).expect(
            "Błąd wejścia/wyjścia",
        );
        let max_time: i64 = max_time.trim().parse().expect("Błędna wartość");

        // Rozwiązanie z parametrami
        tabu_search::solve(&mut matrix,
                           iterations,
                           lifetime,
                           critical_events,
                           max_time);
    }
}
