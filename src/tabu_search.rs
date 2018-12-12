extern crate rand;
extern crate time;
use self::rand::Rng;


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
    let mut current_path_value: i32 = 9999999999;
    // Najlepsza ściezka
    let mut best_path: Vec<i32>;
    // Kosazt najlepszej ścieżki
    let mut best_path_value: i32 = 9999999999;
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

fn get_current_path_value(matrix: &Vec<Vec<i32>>,
                          path: &mut Vec<i32>) -> i32 {

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
    let mut best_path: Vec<i32> = Vec::new();
    let mut current_path_value: i32;
    let mut best_path_value: i32 = 99999999;
    let mut best_city_x: i32 = 0;
    let mut best_city_y: i32 = 0;

    for i in 0..path.len() {
        for j in 0..path.len() {
            if tabu_list[i][j] == 0 {
                current_path = path.clone();
                current_path[i] = path[j];
                current_path[j] = path[i];
                current_path_value = get_current_path_value(matrix, &mut current_path);

                if current_path_value < best_path_value {
                    best_path = current_path;
                    best_path_value = current_path_value;
                    best_city_x = i as i32;
                    best_city_y = j as i32;
                }
            }
        }
    }

    tabu_list[(best_city_x as usize)][(best_city_y as usize)] = lifetime;

    return best_path;
}