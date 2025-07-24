mod calc;

use calc::*;

fn main() {
    let _add: Option<i64> = OperationType::Addition.perform(2, 3);
    println!("The addition is {:#?}", _add);

    let _sub: Option<i64> = OperationType::Subtraction.perform(3, 4);
    println!("The value of subtraction is {:#?}", _sub);

    let _mul: Option<i64> = OperationType::Multiplication.perform(5, 5);
    println!("The value of multiplication is {:#?}", _mul);

    let add_sign = OperationType::Addition;
    let sub_sign: OperationType = OperationType::Subtraction;
    let mul_sign: OperationType = OperationType::Multiplication;
    println!("{}", add_sign.get_sign());
    println!("{}", sub_sign.get_sign());
    println!("{}", mul_sign.get_sign());
}
