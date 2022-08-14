#![allow(non_snake_case)]

pub mod BinOp;
pub mod DataType;
pub mod Draw;
pub mod Expr;
pub mod Symbol;
pub mod Test;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
