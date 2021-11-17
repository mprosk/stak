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
                // STACK MANAGEMENT
                "." => {
                    // Drop the top value
                    let _ = self.stack.pop();
                    Ok(())
                }
                ".." => {
                    // Clear the stack
                    self.stack.clear();
                    Ok(())
                }
                "&" => self.dupe(), // Duplicate the top value
                "<>" => self.swap(), // Swap the top values

                // OPERATORS
                "+" => self.add(),
                "-" => self.sub(),
                "*" => self.mult(),
                "/" => self.div(),
                "^" => self.pow(),
                "**" => self.pow(),
                "%" => self.modulo(),
                "sqrt" => self.sqrt(),
                "log2" => self.log2(),
                "inv" => self.inv(),
                "ceil" => self.ceil(),
                "floor" => self.floor(),
                "abs" => self.abs(),
                "sum" => self.sum(),
                "prod" => self.prod(),

                // CONSTANTS
                "e" => {
                    self.stack.push(std::f64::consts::E);
                    Ok(())
                }
                "pi" => {
                    self.stack.push(std::f64::consts::PI);
                    Ok(())
                }

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

    /// Performs a floor
    fn floor(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let a = self.stack.pop().unwrap();
            self.stack.push(a.floor());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a ceiling
    fn ceil(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let a = self.stack.pop().unwrap();
            self.stack.push(a.ceil());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs an absolute value
    fn abs(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let a = self.stack.pop().unwrap();
            self.stack.push(a.abs());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Clones the top value of the stack
    fn dupe(&mut self) -> Result<(), StakError> {
        if let Some(a) = self.stack.pop() {
            self.stack.push(a);
            self.stack.push(a);
        }
        Ok(())
    }

    /// Swaps the top two values
    fn swap(&mut self) -> Result<(), StakError> {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(a);
            self.stack.push(b);
        }
        Ok(())
    }

    /// Performs a summation of all values in the stack
    fn sum(&mut self) -> Result<(), StakError> {
        let mut sum = 0_f64;
        for x in &self.stack {
            sum += *x;
        }
        self.stack.clear();
        self.stack.push(sum);
        Ok(())
    }

    /// Performs a product of all values in the stack
    fn prod(&mut self) -> Result<(), StakError> {
        let mut prod = 1_f64;
        for x in &self.stack {
            prod *= *x;
        }
        self.stack.clear();
        self.stack.push(prod);
        Ok(())
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
