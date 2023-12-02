#[derive(Debug)]

struct Round {
    red: usize,
    green: usize,
    blue: usize
}

#[derive(Debug)]
struct Game {
    game : usize,
    rounds : Vec<Round>
}

impl TryFrom::<&str> for Round {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut int_colors = Vec::new();
        let colors = vec![value.find("red").unwrap_or(0), value.find("green").unwrap_or(0), value.find("blue").unwrap_or(0)];
        for color in colors {
            if color != 0 {
                int_colors.push(value[(color - 3)..color].trim().parse::<usize>().unwrap());
            } else {
                int_colors.push(0);
            }
        }
        return Ok(Round { red: int_colors[0], green: int_colors[1], blue: int_colors[2] })
    }
}

impl TryFrom::<&str> for Game {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (game, round_collection) = value.split_at(value.find(":").unwrap() + 1);
        let game = &game.replace("Game", "");
        let game = &game.replace(":", "");
        let game = game.trim();
        let rounds: Vec<&str> = round_collection.split(";").collect();
        let mut return_rounds = Vec::new();
        for round in rounds {
            return_rounds.push(Round::try_from(round).unwrap())
        }
        return Ok(Game { game: game.parse::<usize>().unwrap(), rounds: return_rounds })
    }
}

impl Round {
    fn is_possible(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().map(|x| x.is_possible()).all(|x| !!x)
    }

    fn get_minimum(&self) -> usize {
        let (mut red, mut green, mut blue) = (Vec::new(), Vec::new(), Vec::new());
        for round in &self.rounds {
            red.push(round.red);
            green.push(round.green);
            blue.push(round.blue)
        }
        return red.iter().max().unwrap() * green.iter().max().unwrap() * blue.iter().max().unwrap()
    }
}


fn main() {
    let data = include_str!("input").lines();
    let mut possible = 0;
    for game in data {
        let prueba = Game::try_from(game).unwrap();
        let _ = prueba.is_possible();
        possible += prueba.get_minimum();
    }
    println!("{possible:?}");
}
