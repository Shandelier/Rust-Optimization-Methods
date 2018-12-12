extern crate rand;

use self::rand::distributions::Range;
use self::rand::distributions::IndependentSample;

//Generuje macierz ogległości grafu
//Na przekątnej znajdują się wartości -1
//Zwraca macierz w formacie Vec<Vec<i32>>
pub fn generate_random_graph(number_of_nodes: i32) -> Vec<Vec<i32>> {

    //Pusta macierz
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    //Zakres do generacji liczb losowych od 1 do 100
    let rng_range = Range::new(1, 100);
    //Wątek generatora liczb losowych, bez seed'a
    let mut rng = rand::thread_rng();

    //Iterowanie po wierszach macierzy
    for i in 0..number_of_nodes {

        //Zmienna tymczasowa przechowująca powstający w pętli poniżej wiersz macierzy
        let mut row: Vec<i32> = Vec::new();

        //Iterowanie po kolumnach macierzy
        //Na przekątnej wstawia zawsze -1
        //W pozostałych przypadkach rand(1, 100)
        for j in 0..number_of_nodes {
            if i == j {
                row.push(-1);
            } else {
                row.push(rng_range.ind_sample(&mut rng));
            }
        }
        matrix.push(row);
    }
    return matrix;
}
