//
//Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
//
//For example:
//
//For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
//For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
//For a mass of 1969, the fuel required is 654.
//For a mass of 100756, the fuel required is 33583.
//

use std::fs;

fn read_input() -> String {
    let contents = fs::read_to_string("/Users/gtaylor/ersonal/src/advent-of-code/2019/rust/advent_of_code/src/day_one_input")
        .expect("Something went wrong reading the file");
    return contents;
}

pub fn calc_fuel_for_mass(mass: i32) -> i32 {
    return (mass / 3) - 2;
}

pub fn calc_fuel_for_mass_part_2(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if(fuel <= 0){
        return 0;
    }
    return fuel + calc_fuel_for_mass_part_2(fuel);
}

#[cfg(test)]
mod tests {
    use crate::day_one::{calc_fuel_for_mass, read_input, calc_fuel_for_mass_part_2};
    use std::str::FromStr;

    #[test]
    fn get_solution() {
        let input = read_input();

        let total = input.lines().map(|s| calc_fuel_for_mass(u32::from_str(s).unwrap() as i32 )).fold(0, |a, b| a + b);

        assert_eq!(total, 2);
    }

    #[test]
    fn get_solution_part_two() {
        let input = read_input();

        let total = input.lines().map(|s| calc_fuel_for_mass_part_2(u32::from_str(s).unwrap() as i32 )).fold(0, |a, b| a + b);

        assert_eq!(total, 2);
    }

    #[test]
    fn test_input() {
        let input = read_input();

        let a = vec![12, 14];

        let total = a.into_iter().map(|n| calc_fuel_for_mass(n )).fold(0, |a, b| a + b);

        assert_eq!(total, 4);
    }

    #[test]
    fn solve_mass_example_one() {
        let solution = calc_fuel_for_mass(12);

        assert_eq!(solution, 2);
    }

    #[test]
    fn solve_mass_example_one_part_two() {
        let solution = calc_fuel_for_mass_part_2(1969);

        assert_eq!(solution, 966);
    }

    #[test]
    fn solve_mass_example_two() {
        let input = "";

        let solution = calc_fuel_for_mass(14);
        //282
        assert_eq!(solution, 2);
    }

    #[test]
    fn solve_mass_example_three() {
        let input = "";

        let solution = calc_fuel_for_mass(1969);
        //282
        assert_eq!(solution, 654);
    }

    #[test]
    fn solve_mass_example_four() {
        let input = "";

        let solution = calc_fuel_for_mass(100756);
        //282
        assert_eq!(solution, 33583);
    }
}