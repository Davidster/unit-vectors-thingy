const ITERATIONS_PER_THREAD: usize = 1_000_000_000;
const EPSILON: f32 = 0.01;

fn random_float() -> f32 {
    bytemuck::cast(fastrand::u32(..))
}

fn get_num_normals() -> usize {
    let mut count = 0;
    for _ in 0..ITERATIONS_PER_THREAD {
        let random_vec: [f32; 3] = [random_float(), random_float(), random_float()];
        let norm = (random_vec[0] * random_vec[0]
            + random_vec[1] * random_vec[1]
            + random_vec[2] * random_vec[2])
            .sqrt();
        if norm > 1.0 - EPSILON && norm < 1.0 + EPSILON {
            count += 1;
        }
    }
    count
}

fn main() {
    let num_threads = num_cpus::get();

    let mut threads: Vec<_> = (0..num_threads)
        .map(|_| std::thread::spawn(|| get_num_normals()))
        .collect();

    let num_normals = threads
        .drain(..)
        .map(|thread| thread.join().unwrap())
        .fold(0, |acc, val| acc + val);

    let iterations = ITERATIONS_PER_THREAD * num_threads;
    println!(
        "Found {} normal vectors out of {} possibilities, {:.4}% are not normals",
        num_normals,
        pretty_print_int(iterations),
        100.0 * ((iterations - num_normals) as f32 / iterations as f32)
    );
}

fn pretty_print_int(i: usize) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
