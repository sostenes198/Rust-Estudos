use rand::Rng;

fn generate_random_numbers(min: i32, max: i32, quantity: i32) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..quantity).map(|_| rng.random_range(min..=max)).collect()
}

fn main() {
    let mut random_array = generate_random_numbers(0, 100, 10);

    println!("Array before sorting: {:?}", random_array);

    for i in 1..random_array.len() {
        let key = random_array[i];
        let mut j = i as isize - 1;
        //  Inserir A[i] no subvetor ordenado A[1: i – 1].
        // Para alterar a ordenação mude (> key) para (< key)
        while j >= 0 && random_array[j as usize] > key {
            random_array[(j + 1) as usize] = random_array[j as usize];
                j = j - 1;
        }
        random_array[(j + 1) as usize] = key;
    }

    println!("Array after sorting: {:?}", random_array);
}
