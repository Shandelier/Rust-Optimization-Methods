//Drukuje na ekran macierz
pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!("Macierz grafu:");

    for i in 0..matrix.len() {
        println!("{:?}", matrix[i as usize]);
    }
}

pub fn print_tabu_list(tabu_list: &Vec<Vec<i32>>) {
    println!("Lista tabu:");

    for i in 0..tabu_list.len() {
        println!("{:?}", tabu_list[i as usize]);
    }
}

pub fn print_result(result: i32, result_path: Vec<i32>, elapsed_time: i64) {
    println!();
    println!("****************************************************************");
    println!("Trasa: {:?}", result_path);
    println!("Koszt: {}", result);
    println!("Czas: {}ns", elapsed_time);
    println!("****************************************************************");
    println!();
}