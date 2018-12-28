#[derive(Debug)]
struct Scores {
    values: Vec<Score>,
    players: Vec<Player>,
}

impl Scores {
    fn push(&mut self, score: Score) {
        self.values.push(score);
    }

    fn next(&mut self) {
        let mut new_recipe_score = 0;
        for player in &self.players {
            let score = &self.values[player.current_index];
            new_recipe_score += score.value;
        }

        let new_recipe_score = NewRecipeScore::new(new_recipe_score);
        let transform_scores = new_recipe_score.to_score();

        for score in transform_scores {
            self.push(score);

            let number_of_receipts = 864801;
            let number = 6;

            if self.values.len() >= number {
                let mut multiplier = 100000;
                let mut res = 0;
                for score in &self.values[self.values.len() - number..] {
                    res += multiplier * score.value as usize;
                    multiplier /= 10;
                }

                // println!("res {}", res);

                if res == number_of_receipts {
                    panic!("after {} recipes", self.values.len() - number);
                }
            }
        }

        for player in &mut self.players {
            player.next_index(&self.values[player.current_index], self.values.len());
        }
    }
}

struct NewRecipeScore {
    value: u8,
}

impl NewRecipeScore {
    fn new(value: u8) -> NewRecipeScore {
        if value > 18 {
            panic!("Not expected value for NewRecipeScore {}", value);
        }

        NewRecipeScore { value }
    }

    fn to_score(self) -> Vec<Score> {
        fn int_to_scores(n: u8) -> Vec<Score> {
            fn x_inner(n: u8, xs: &mut Vec<Score>) {
                if n >= 10 {
                    x_inner(n / 10, xs);
                }
                xs.push(Score { value: n % 10 });
            }
            let mut xs = Vec::new();
            x_inner(n, &mut xs);
            xs
        }

        let scores = int_to_scores(self.value);
        scores
    }
}

#[derive(Debug)]
struct Score {
    value: u8,
}

impl Score {
    fn new(value: u8) -> Score {
        if value > 9 {
            panic!("Not expected value {}", value);
        }

        Score { value }
    }
}

#[derive(Debug)]
struct Player {
    index: u8,
    current_index: usize,
}

impl Player {
    fn next_index(&mut self, current_value: &Score, len_of_scores: usize) {
        self.current_index = 1 + current_value.value as usize + self.current_index;

        loop {
            if self.current_index >= len_of_scores {
                self.current_index -= len_of_scores;
            } else {
                break;
            }
        }
        // println!("current_index {} player {}", self.current_index, self.index);
    }
}

fn main() {
    let mut scores = Scores {
        values: vec![Score::new(3), Score::new(7)],
        players: vec![
            Player {
                index: 0,
                current_index: 0,
            },
            Player {
                index: 1,
                current_index: 1,
            },
        ],
    };

    loop {
        scores.next();
    }
}
