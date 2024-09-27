use rand::Rng;

fn get_random_num() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..69.0)
}

fn cost(w: f64, training_data: &[Vec<f64>]) -> f64 {
    let mut result = 0.0;
    for data_point in training_data {
        let x = data_point[0];
        let expected = data_point[1];
        let y = x * w;
        let d = y - expected;
        result += d * d;
    }
    result / training_data.len() as f64
}

fn main() {
    let training_data = vec![
        vec![0.0, 0.0],
        vec![1.0, 2.0],
        vec![2.0, 4.0],
        vec![3.0, 6.0],
        vec![4.0, 8.0],
    ];

    let mut w = get_random_num();
    println!("W0--> {}", w);
    let eps = 1e-3;
    let rate = 1e-3;
    for i in 0..200 {
        let dcost: f64 = (cost(w + eps, &training_data) - cost(w, &training_data)) / eps;
        w -= rate * dcost;
        println!("Iteration {}: Cost: {:.4}", i, cost(w, &training_data));
    }

    println!("Final weight: {:.4}", w);
}
