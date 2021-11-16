use crate::stak::StakError::{InvalidToken, StackEmpty};
use thiserror::Error;

/// Container structure for the RPN calculator engine
#[derive(Debug)]
pub struct Stak {
    stack: Vec<f64>,
}

impl Stak {
    /// Creates a new Stak structure
    pub fn new() -> Stak {
        let stack: Vec<f64> = Vec::new();
        Stak { stack }
    }

    /// Prints the contents of the stack
    pub fn print_stack(&self) {
        println!("{:?}", self.stack)
    }

    /// Parses the given token and performs the appropriate action
    /// `token` must be a single token with no leading or trailing whitespace
    /// Returns an error, or the optional top value of the stack
    pub fn parse_token(&mut self, token: &str) -> Result<(), StakError> {
        // Check if the token is a number
        if let Ok(val) = token.parse::<f64>() {
            self.stack.push(val);
        } else {
            return match token {
                // Drop the top value
                "." => {
                    let _ = self.stack.pop();
                    Ok(())
                }
                // Clear the stack
                ".." => {
                    self.stack.clear();
                    Ok(())
                }

                // Addition
                "+" => self.add(),
                // Subtraction
                "-" => self.sub(),
                // Multiplication
                "*" => self.mult(),
                // Division
                "/" => self.div(),
                // Power
                "^" => self.pow(),
                // Modulo
                "%" => self.modulo(),
                // Square Root
                "sqrt" => self.sqrt(),
                // Log base 2
                "log2" => self.log2(),
                // Inversion
                "inv" => self.inv(),

                _ => Err(InvalidToken(token.to_string())),
            };
        }
        Ok(())
    }

    /// Performs an addition
    fn add(&mut self) -> Result<(), StakError> {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(a + b);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a subtraction
    fn sub(&mut self) -> Result<(), StakError> {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(b - a);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a multiplication
    fn mult(&mut self) -> Result<(), StakError> {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(a * b);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a division
    fn div(&mut self) -> Result<(), StakError> {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(b / a);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs an exponent
    fn pow(&mut self) -> Result<(), StakError> {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(b.powf(a));
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a modulo
    fn modulo(&mut self) -> Result<(), StakError> {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(b % a);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a square root
    fn sqrt(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let a = self.stack.pop().unwrap();
            self.stack.push(a.sqrt());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a log base 2
    fn log2(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let a = self.stack.pop().unwrap();
            self.stack.push(a.log2());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs an inversion
    fn inv(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let a = self.stack.pop().unwrap();
            self.stack.push(1_f64 / a);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }
}

// Errors
#[derive(Error, Debug)]
pub enum StakError {
    #[error("invalid token `{0}`")]
    InvalidToken(String),
    #[error("not enough values on the stack")]
    StackEmpty,
}
