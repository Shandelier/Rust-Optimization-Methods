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
        println!("1. Wczytaj graf z pliku");
        println!("2. Generuj graf");
        println!("3. Tabu Search");
        println!("4. Symulowane Wyżarzanie");
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
                    tabu_search::prepare(&mut matrix)
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


}
