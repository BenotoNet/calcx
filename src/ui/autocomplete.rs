pub fn get_options() -> Vec<String> {
"sqrt(, pi, speed_of_light
kilo, meter"
        .split(&[' ', ',', '\n'])
        .into_iter()
        .map(|v| {v.to_string()})
        .collect()
}
