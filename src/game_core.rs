use rand::Rng;

pub struct GameCore {
    secret_num: Vec<u32>,
    user_answer: Vec<u32>,
}

impl GameCore {
    pub fn new() -> GameCore {
        GameCore {
            secret_num: vec![],
            user_answer: vec![],
        }
    }

    pub fn generate(&mut self) {
        self.secret_num =
            GameCore::to_vec_digit(&format!("{:03}", rand::thread_rng().gen_range(0..1000)));
    }

    pub fn set_user_answer(&mut self, input: &str) {
        self.user_answer = GameCore::to_vec_digit(&input[0..3]);
    }

    pub fn eq_pos_count(&self) -> u32 {
        let mut count = 0;
        for idx in 0..self.secret_num.len() {
            if self.secret_num[idx] == self.user_answer[idx] {
                count += 1;
            }
        }
        count
    }

    pub fn eq_nums_count(&self) -> u32 {
        let mut count = 0;
        for snum in &self.secret_num {
            for unum in &self.user_answer {
                if unum == snum {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    pub fn check_win(&self) -> bool {
        if self.eq_nums_count() == 3 && self.eq_pos_count() == 3 {
            return true;
        }
        false
    }

    fn to_vec_digit(input: &str) -> Vec<u32> {
        input
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }

    // debgu only
    pub fn get_secret(&self) -> &[u32] {
        &self.secret_num
    }

    // debug only
    pub fn get_user_answer(&self) -> &[u32] {
        &self.user_answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // secret_num vec len always must be 3
    #[test]
    fn test_generator() {
        let mut core = GameCore::new();
        for _ in 0..30000 {
            core.generate();
            assert_eq!(core.get_secret().len(), 3)
        }
    }
}
