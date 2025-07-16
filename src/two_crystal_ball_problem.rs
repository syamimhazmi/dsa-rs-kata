fn two_crystal_balls(breaks: &[bool]) -> i32 {
    let jump_amount = (breaks.len() as f64).sqrt().floor() as usize;

    println!("jumping amount: {}", jump_amount);

    let mut starting_point = jump_amount;

    println!("starting point: {}", starting_point);

    // first ball validation of breaks
    while starting_point < breaks.len() {
        if breaks[starting_point] {
            println!("found start point of true: {}", starting_point);
            break;
        }

        println!("starting point is false: {}", starting_point);

        starting_point += jump_amount;

        println!("increased amount of start point: {}", starting_point);
    }

    starting_point -= jump_amount;

    println!("decrease starting point: {}", starting_point);

    let mut j = 0;

    // second ball validation of breaks
    while j <= jump_amount && starting_point < breaks.len() {
        if breaks[starting_point] {
            println!("return the value: {}", starting_point);
            return starting_point as i32;
        }

        println!("we haven't found the value yet: {}", starting_point);

        j += 1;
        starting_point += 1;

        println!(
            "increase the value of last truth point: {} and start point: {}",
            j, starting_point
        );
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::two_crystal_balls;

    #[test]
    fn tests_breaks_at_floor_5() {
        let breaks1 = [
            false, false, false, false, false, true, true, true, true, true,
        ];
        assert_eq!(two_crystal_balls(&breaks1), 5);
    }

    #[test]
    fn it_doesnt_break() {
        let breaks2 = [false, false, false, false, false, false, false, false];

        assert_eq!(two_crystal_balls(&breaks2), -1);
    }

    #[test]
    fn it_breaks_at_ground_floor() {
        let breaks3 = [true, true, true, true, true];

        assert_eq!(two_crystal_balls(&breaks3), 0);
    }

    #[test]
    fn it_breaks_at_the_top_flor() {
        let breaks4 = [
            false, false, false, false, false, false, false, false, false, true,
        ];

        let result = two_crystal_balls(&breaks4);

        assert_eq!(result, 9);
    }
}
