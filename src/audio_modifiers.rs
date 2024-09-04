pub(crate) fn deepen_voice(input: &[f32]) -> Box<[f32]> {
    let intensity: f32 = 0.5f32;
    let len: usize = input.len();
    let mut result: Vec<f32> = Vec::with_capacity(len);

    for (index, &value) in input.iter().enumerate() {
        if index < len / 2 {
            result.push(value * intensity);
        } else {
            result.push(value / intensity);
        }
    }

    result.into_boxed_slice()
}

pub(crate) fn low_pass_filter(input: &Vec<f32>, alpha: f32) -> Vec<f32> {
    let mut output = vec![0.0; input.len()];

    output[0] = input[0]; // Initialize the first sample

    for i in 1..input.len() {
        output[i] = (alpha * input[i] + (1.0 - alpha) * output[i - 1]);
    }

    output
}

pub(crate) fn low_pass_filter_threshold(input: &Vec<f32>, alpha: f32, mut threshold: usize) -> Vec<f32> {
    if threshold > input.len() {
        threshold = input.len();
    }

    let mut low_pass_output = vec![0.0; input.len()];
    let mut high_pass_output = vec![0.0; input.len()];

    low_pass_output[0] = input[0]; // Initialize the first sample

    for i in 1..(threshold-1) {
        low_pass_output[i] = alpha * input[i] + (1.0 - alpha) * low_pass_output[i - 1];
        high_pass_output[i] = input[i] - low_pass_output[i];
    }

    for i in threshold..input.len() {
        high_pass_output[i] = input[i];
    }

    // println!("{}\t{}", &input.len(), &high_pass_output.len());
    high_pass_output
}
