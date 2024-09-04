pub(crate) fn low_pass_filter(input: &Vec<f32>, alpha: f32) -> Vec<f32> {
    let mut output = vec![0.0; input.len()];

    output[0] = input[0];

    for i in 1..input.len() {
        output[i] = alpha * input[i] + (1.0 - alpha) * output[i - 1];
    }

    output
}
