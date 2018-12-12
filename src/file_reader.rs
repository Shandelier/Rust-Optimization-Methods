extern crate tsplib;

use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::fs::File;

pub fn read_any_file(file_name: String) -> Vec<Vec<i32>> {
    if file_name.ends_with("txt") {
        println!("Wybrano macierz grafu");
        return read_txt_file(file_name)
    } else if file_name.ends_with("atsp") {
        println!("Wybrano plik ATSP");
        return read_atsp_file(file_name)
    } else {
        println!("Wybrano plik TSP");
        return read_tsp_file(file_name)
    }
}

//Odczytuje macierz z pliku
pub fn read_txt_file(file_name: String) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    println!("Odczytywanie pliku {}...", &file_name);

    //Stworzenie zmiennej plikowej
    let file = match File::open(&file_name) {
        Err(error) => {
            panic!(
                "At the Disco! Couldn't open {}: {}",
                &file_name,
                Error::description(&error)
            )
        }
        Ok(file) => file,
    };

    //Stworzenie bufora ze zmiennej plikowej
    let buffer = BufReader::new(&file);

    //Zmienna określająca ilość wczytanych linii
    //Potrzebna aby wygodnie numerować tablice od 0
    let mut first_line: bool = true;

    //Iteracja po kolejnych liniach pliku
    for line in buffer.lines() {
        //Pomiń linie bez tekstu, uszkodzone itd.
        match line {
            Ok(line) => {
                //Wyswietl liczbę miast z pierwszej linii
                //Pozostałe linie parsuj do wektora i dodaj jako wiersz macierzy
                if first_line {
                    println!("Liczba miast: {}", line);
                    first_line = false;
                } else {
                    matrix.push(parse_file_line(line));
                }
            }
            Err(error) => println!("Błąd pliku: {}", error),
        }
    }

    return matrix;
}

//Parsuje string z pliku do postaci wektora liczb i32
fn parse_file_line(line: String) -> Vec<i32> {
    let cleared_line = line.replace("/  +/", " ");

    return cleared_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}

fn read_tsp_file(file_name: String) -> Vec<Vec<i32>> {
    let instance_result = self::tsplib::read(file_name);
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    match instance_result {
        Ok(instance) => {
            println!("Nazwa: {}", &instance.name);

            let node_coord_option = instance.node_coord;

            match node_coord_option {
                Some(node_coords) => {
                    match node_coords {
                        self::tsplib::NodeCoord::Two(nodes_2d) => {
                            matrix = convert_coordinates_to_weigth_matrix(nodes_2d);
                        }

                        self::tsplib::NodeCoord::Three(_nodes_3d) => {
                            println!("Algorytm nie wspiera grafow 3d");
                        }
                    }
                }

                None => println!("Brak koordynatów"),
            }
        }

        Err(error) => {
            println!("Błąd: {}", &error)
        }
    }
    return matrix;
}

fn convert_coordinates_to_weigth_matrix(nodes_2d: Vec<(usize, f32, f32)>) -> Vec<Vec<i32>> {
    let node_list = nodes_2d;
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for i in 0..node_list.len() {
        let mut matrix_row: Vec<i32> = Vec::new();
        let x_start = node_list[i].1;
        let y_start = node_list[i].2;

        for j in 0..node_list.len() {
            let x_end = node_list[j].1;
            let y_end = node_list[j].2;


            if i == j {
                matrix_row.push(-1);
            } else {
                matrix_row.push(distance_between_coordinates(x_start, y_start, x_end, y_end));
            }
        }

        matrix.push(matrix_row);
    }

    return matrix;
}

fn distance_between_coordinates(x_start: f32,
                                y_start: f32,
                                x_end: f32,
                                y_end: f32) -> i32 {
    let distance_power_sum = (x_start - x_end).powi(2) + (y_start - y_end).powi(2);

    return distance_power_sum.sqrt() as i32;
}

fn read_atsp_file(file_name: String) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    //Stworzenie zmiennej plikowej
    let file = match File::open(&file_name) {
        Err(error) => {
            panic!(
                "At the Disco! Couldn't open {}: {}",
                &file_name,
                Error::description(&error)
            )
        }
        Ok(file) => file,
    };

    //Stworzenie bufora ze zmiennej plikowej
    let buffer = BufReader::new(&file);

    //Zmienna określająca ilość wczytanych linii
    //Potrzebna aby wygodnie numerować tablice od 0
    let mut current_line: usize = 0;

    //Zmienna przechowująca liczbę miast
    let mut number_of_nodes: i32 = 0;

    // Pojedyńczy rząd macierzy wag
    let mut matrix_row: Vec<i32> = Vec::new();

    //Iteracja po kolejnych liniach pliku
    for line in buffer.lines() {

        //Pomiń linie bez tekstu, uszkodzone itd.
        match line {
            Ok(line) => {
                //Pobierz liczbę miast z trzeciej linii
                if current_line == 3 {
                    let number_of_cities: Vec<&str> = line.split_whitespace().collect();
                    number_of_nodes = number_of_cities[1].parse().unwrap();
                    println!("Liczba miast: {}", number_of_nodes);
                } else

                // Jeżeli linia jest linią macierzy i nie jest EOF
                if current_line > 6 && !line.contains("EOF") {

                    // Parsuj linię do wektora
                    let parsed_line = parse_file_line(line);

                    // Dodaj wszytkie elementy z linii do wiersza macierzy
                    for i in 0..(parsed_line.len() as usize) {
                        matrix_row.push(parsed_line[i]);
                    }

                    // Dodaj gotowy wiersz macierzy
                    // Wyczyść zmienną
                    if matrix_row.len() >= number_of_nodes as usize {
                        matrix.push(matrix_row.clone());
                        matrix_row.clear();
                    }
                }
            }
            Err(error) => println!("Błąd pliku: {}", error),
        }

        current_line = current_line + 1;
    }

    return matrix;
}



