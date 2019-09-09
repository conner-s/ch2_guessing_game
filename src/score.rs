pub mod scoring {
    pub fn get_score(list: Vec<u32>, magic_num: u32) -> i64 {
        let mut score: i64 = 0;
        for index in 0..5 {
            let working_score =(list[index] - magic_num) as i64;
            score += 50 - working_score.abs();
        }
        score
    }
}