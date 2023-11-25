pub fn get_score(list: Vec<u32>, magic_num: u32) -> i32 {
    let mut score: i32 = 0;
    for index in 0..5 {
        let working_score =list[index] as i32 - magic_num as i32;
        score += 50 - working_score.abs();
    }
    score
}
