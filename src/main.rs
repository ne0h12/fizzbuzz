enum MultipleUnit {
    Five,
    Three,
    ThreeAndFive,
    Other
}

struct Calculator;
impl Calculator {
    fn multiple(digit: i32) -> MultipleUnit {
        let remainderOfDivisionByThreeEqualZero = digit % 3 == 0;
        let remainderOfDivisionByFiveEqualZero = digit % 5 == 0;

        if remainderOfDivisionByThreeEqualZero &&
           remainderOfDivisionByFiveEqualZero {
            return MultipleUnit::ThreeAndFive;
        } else if remainderOfDivisionByThreeEqualZero {
            return MultipleUnit::Three;
        } else if remainderOfDivisionByFiveEqualZero {
            return MultipleUnit::Five;
        } else {
            return MultipleUnit::Other;
        }
    }
}

fn main() {
    for number in 1..101 {
        match Calculator::multiple(number) {
            MultipleUnit::Three => println!("fizz"),
            MultipleUnit::Five => println!("buzz"),
            MultipleUnit::ThreeAndFive => println!("fizzbuzz"),
            MultipleUnit::Other => println!("{}", number)
        }
    }
}
