extern crate rand;
extern crate time;

use self::rand::Rng;
use std::io;

pub fn prepare(mut matrix: &mut Vec<Vec<i32>>) {
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
    solve(&mut matrix,
          iterations,
          lifetime,
          critical_events,
          max_time);
}

pub fn solve(matrix: &mut Vec<Vec<i32>>,
             iterations: i32,
             lifetime: i32,
             max_critical_events: i32,
             max_time_in_seconds: i64) {
    println!("Przygotowywanie zmiennych...");

    // Początek zliczania czasu
    let timer_start = time::PreciseTime::now();
    // Ilość istotnych zdarzeń
    let mut critical_events: i32 = 0;
    // Aktualna ściezka
    let mut current_path: Vec<i32> = Vec::new();
    // Koszt aktualnej ściezki
    let mut current_path_value: i32 = <i32>::max_value();
    // Najlepsza ściezka
    let mut best_path: Vec<i32>;
    // Kosazt najlepszej ścieżki
    let mut best_path_value: i32 = <i32>::max_value();
    // Lista tabu
    let mut tabu_list: Vec<Vec<i32>>;

    // Wypełnij wektor okreslający ściezkę kolejnymi wierzchokami
    for i in 0..(matrix.len() as i32) {
        current_path.push(i);
    }

    // Losowa zmiana kolejności w wektorze
    rand::thread_rng().shuffle(&mut current_path);

    // Sprawdzenie kosztu aktualnej drogi
    current_path_value = get_current_path_value(&matrix, &mut current_path);

    // Wygeneorwanie pustej listy tabu
    tabu_list = generate_empty_tabu_list(matrix.len() as i32);

    // Przypisanie początkowych wartości ściezki jako najlepszych znalezionych
    //K: użyj .clone() aby current_path nie straciło ownershipa
    best_path = current_path.clone();
    best_path_value = current_path_value.clone();

    println!("Początek algorytmu...");


    // Pętla wykonująca zadaną ilość iteracji
    for _i in 0..iterations {

        // Obliczenie aktualnego czasu
        let elapsed_time = timer_start
            .to(time::PreciseTime::now())
            .num_nanoseconds()
            .unwrap();

        // Warunek kończący czasowo
        // Jeżeli czas jest ustawiony na inny niż 0s
        // Pętla przerywa się po wybranym czasie
        if (elapsed_time >= (max_time_in_seconds * 1000000000)) && max_time_in_seconds != 0 {
            println!("Przekroczono czas");
            break;
        }

        // Zmienna przechowująca koszt scieżki badanej w aktualnej iteracji
        let iteration_path_value = current_path_value.clone();

        // Zmiana elementów w ścieżce
        current_path = swap_elements(&mut current_path, &mut tabu_list, &matrix, lifetime);

        // Aktualizacja wagi aktualnej ściezki
        current_path_value = get_current_path_value(&matrix, &mut current_path);

        // Przypisanie wartości aktualnej ścieżki jako najlepszej
        if current_path_value < best_path_value {
            println!();
            best_path = current_path.clone();
            best_path_value = current_path_value.clone();
            // Obliczenie aktualnego czasu
            let elapsed_time = timer_start
                .to(time::PreciseTime::now())
                .num_nanoseconds()
                .unwrap();

            print!("{} {}", best_path_value, elapsed_time);
        }

        // Minimalizacja wartości w liście tabu
        for i in 0..matrix.len() {
            for j in 0..matrix.len() {
                if tabu_list[i][j] > 0 {
                    tabu_list[i][j] = tabu_list[i][j] - 1;
                }
            }
        }

        // Jeżeli wynik nie jest poprawny, zwiększenie licznika krytycznych błędów
        if iteration_path_value > current_path_value {
            critical_events = critical_events + 1;
        } else {
            critical_events = 0;
        }

        // Zmiana zbioru i oczyszczenie listy tabu
        // Jeżeli przekroczona została ilość błędów krytycznych
        // Podanie 0 jako maks zdarzeń krytycznych powoduje wyłączenie dywersyfkacji
        if (critical_events >= max_critical_events) && (max_critical_events != 0) {
            println!();
            println!("Dywersyfikacja");
            rand::thread_rng().shuffle(&mut current_path);
            tabu_list = generate_empty_tabu_list(matrix.len() as i32)
        }
    }

    println!();
    println!("Najlepsza ściezka: {:?}", best_path);
    println!("Koszt najlepszej ścieżki: {}", best_path_value);
}

fn generate_empty_tabu_list(size: i32) -> Vec<Vec<i32>> {
    let mut tabu_list: Vec<Vec<i32>> = Vec::new();

    for _i in 0..size {
        let mut tabu_list_row: Vec<i32> = Vec::new();
        for _i in 0..size {
            tabu_list_row.push(0);
        }
        tabu_list.push(tabu_list_row);
    }

    return tabu_list;
}

pub fn get_current_path_value(matrix: &Vec<Vec<i32>>,
                          path: &Vec<i32>) -> i32 {

    // Początkowy koszt ścieżki to 0
    let mut value: i32 = 0;
    // Pierwszy wierzchołek
    let mut previous_node: usize = 0;

    // Iteracja po wszystkich kolejnych wierzchołkach
    for i in 0..(path.len() as i32) {
        // Zwiększenie kosztu
        value = value + matrix[previous_node][(path[(i as usize)] as usize)];
        // Przypisanie aktualnego wierzchołka jako poprzedniego
        previous_node = path[(i as usize)] as usize;
    }

    // Zwiększenie kosztu trasy o koszt powrotu do wierzchołka początkowego
    value = value + matrix[previous_node][0];

    return value;
}

fn swap_elements(path: &mut Vec<i32>,
                 tabu_list: &mut Vec<Vec<i32>>,
                 matrix: &Vec<Vec<i32>>,
                 lifetime: i32) -> Vec<i32> {
    let mut current_path: Vec<i32>;
    let mut best_path: Vec<i32> = path.clone();
    let mut current_path_value: i32;
    let mut best_path_value: i32 = get_current_path_value(&matrix, &best_path);
    let mut best_city_x: usize = 0;
    let mut best_city_y: usize = 0;

    for i in 0..path.len() {
        for j in 0..path.len() {
            // Nie wolno przechodzić z miasta x do miasta x.
            if i == j {
                continue;
            }

            if tabu_list[i][j] == 0 {
                current_path = path.clone();

                let mut city_a_index: usize = 0;
                let mut city_b_index: usize = 0;

                for m in 0..path.len() {
                    if path[m] == (i as i32) {
                        city_a_index = m;
                    }
                    if path[m] == (j as i32) {
                        city_b_index = m;
                    }
                }
                current_path[city_a_index] = j as i32;
                current_path[city_b_index] = i as i32;

                current_path_value = get_current_path_value(matrix, &mut current_path);

                if current_path_value < best_path_value {
                    best_path = current_path;
                    best_path_value = current_path_value;
                    best_city_x = i;
                    best_city_y = j;
                }
            }
        }
    }

    tabu_list[best_city_x][best_city_y] = lifetime;

    return best_path;
}

mod ts_tests {
    #[test]
    pub fn test_swap_elements() {
        use tabu_search;

        let mut path: Vec<i32> = vec![4, 3, 2, 1, 0];
        let mut tabu_list: Vec<Vec<i32>> = vec![vec![0, 0, 0, 0, 0],
                                                vec![0, 0, 0, 0, 0],
                                                vec![0, 0, 0, 0, 0],
                                                vec![0, 0, 0, 0, 0],
                                                vec![0, 0, 0, 0, 0]];

        let mut matrix: Vec<Vec<i32>> = vec![vec![-1, 27, 57, 59, 55],
                                             vec![1, -1, 21, 31, 53],
                                             vec![55, 17, -1, 69, 18],
                                             vec![71, 47, 53, -1, 59],
                                             vec![83, 17, 57, 95, -1]];


        let lifetime: i32 = 5;

        let path_cost = tabu_search::get_current_path_value(&matrix, &path);
        for i in 0..1000 {
            let result = tabu_search::swap_elements(&mut path, &mut tabu_list, &matrix, lifetime);
            let result_cost = tabu_search::get_current_path_value(&matrix, &result);
            eprintln!("[{}] {:?} -> {}", i, result, result_cost);

            assert_eq!(true, result_cost <= path_cost);
        }
    }
}