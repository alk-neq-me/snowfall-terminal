use std::{process::Command, thread, time::Duration};

use rand::{thread_rng, Rng, seq::SliceRandom};


#[derive(Debug)]
enum Error {
    UnknownTerminalSize,
    UnknownDevice
}


struct Terminal {
    width: usize,
    height: usize
}

impl Terminal {
    fn try_new() -> Result<Self, Error> {
        match term_size::dimensions() {
            Some((width, height)) => Ok(Terminal { width, height }),
            None => Err(Error::UnknownTerminalSize)
        }
    }
}


struct Config {
    density: f64,
    delay: f64,
    snow_flakes: [&'static str; 4]
} 

impl Config {
    fn new(density: f64, delay: f64, snow_flakes: [&'static str; 4]) -> Self {
        Config { density, delay, snow_flakes }
    }
}


struct Grid<'a>(Vec<Vec<&'a str>>);

impl<'a> Grid<'a> {
    fn new(terminal: &Terminal) -> Self {
        let Terminal { width, height } = terminal;
        let mut grid: Vec<Vec<&'a str>> = Vec::new();

        for _ in 0..height.to_owned() {
            grid.push(vec![" "; width.to_owned()]);
        }

        Grid(grid)
    }
}


struct Snowfall<'a> {
    config: &'a Config,
    terminal: &'a Terminal
}

impl<'a> Snowfall<'a> {
    fn new(config: &'a Config, terminal: &'a Terminal) -> Self {
        Snowfall { config, terminal }
    }

    fn draw(&self, grid: Vec<Vec<&str>>) -> Result<(), Error> {
        self.clean_terminal()?;

        println!("\033[?25l");

        let mut output = String::new();

        for row in grid {
            output.push_str(&(row.iter().cloned().collect::<String>() + "\n"));
        }

        output = output.trim_end_matches("\n").to_owned();

        print!("{}", output);

        Ok(())
    }

    fn run(&self, grid: &mut Vec<Vec<&str>>) -> Result<(), Error> {
        loop {
            let mut row: Vec<&str> = vec![];

            let mut rng = thread_rng();

            for _ in 0..(self.terminal.width).to_owned() {
                let random: f64 = thread_rng().gen();
                if random < (self.config.density/100f64) {
                    let snow = self.config.snow_flakes.choose(&mut rng).expect("Failed to get random snow flake.").to_owned();
                    row.push(snow);
                } else {
                    row.push(" ");
                }
            }

            grid.insert(0, row);
            grid.pop();

            self.draw(grid.clone())?;
            thread::sleep(Duration::from_secs_f64(self.config.delay))
        }
    }

    fn clean_terminal(&self) -> Result<(), Error> {
        if cfg!(unix) {
            Command::new("clear")
                .status().map_err(|_| Error::UnknownDevice)?;
        } else if cfg!(windows) {
            Command::new("cmd")
                .arg("/C")
                .arg("cls")
                .status().map_err(|_| Error::UnknownDevice)?;
        }

        Ok(())
    }
}


fn main() -> Result<(), Error> {
    let config = Config::new(7.0, 0.3, ["❆", "❅", "⋆", "•"]);

    let terminal = Terminal::try_new()?;
    let snowfall = Snowfall::new(&config, &terminal);
    let Grid(mut grid) = Grid::new(&terminal);

    snowfall.run(&mut grid)?;

    Ok(())
}
