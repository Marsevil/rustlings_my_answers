// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Eq, Debug, Clone)]
#[repr(usize)]
enum AnimalIdentifier {
    UNKNOWN = 0,
    CRAB = 1,
    GOPHER = 2,
    SNAKE = 3,
}

impl Default for AnimalIdentifier {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl From<usize> for AnimalIdentifier {
    fn from(value: usize) -> Self {
        match value {
            1 => AnimalIdentifier::CRAB,
            2 => AnimalIdentifier::GOPHER,
            3 => AnimalIdentifier::SNAKE,
            _ => AnimalIdentifier::UNKNOWN,
        }
    }
}

impl From<AnimalIdentifier> for usize {
    fn from(value: AnimalIdentifier) -> Self {
        match value {
            AnimalIdentifier::UNKNOWN => 0,
            AnimalIdentifier::CRAB => 1,
            AnimalIdentifier::GOPHER => 2,
            AnimalIdentifier::SNAKE => 3,
        }
    }
}

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        AnimalIdentifier::CRAB
    } else if animal == "gopher" {
        AnimalIdentifier::GOPHER
    } else if animal == "snake" {
        AnimalIdentifier::SNAKE
    } else {
        AnimalIdentifier::default()
    };

    let identifier: isize = <AnimalIdentifier as Into<usize>>::into(identifier) as isize;

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
