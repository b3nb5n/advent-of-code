fn main() {
    let elves = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];

    let mut calories = Vec::<i32>::new();
    for elf in elves.iter() {
        let mut sum = 0;
        for calorie_count in elf.iter() {
            sum += *calorie_count;
        }

        calories.push(sum);
    }

    let mut max = 0;
    for calorie_count in calories.iter() {
        if max < *calorie_count {
            max = *calorie_count
        }
    }

    println!("max: {:?}", max)
}
