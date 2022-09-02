use crate::number::Number;
use crate::stak::StakError::{IndexOutOfRange, InvalidIndex, InvalidToken, StackEmpty};
use thiserror::Error;

/// Container structure for the RPN calculator engine
#[derive(Debug)]
pub struct Stak {
    stack: Vec<f64>,
}

// impl Display for Stak {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut s = String::new();
//         for val in &self.stack {
//             s = format!("{}{}, ", s, Number::f64_to_string(*val));
//         }
//         write!(f, "[{}]", s)
//     }
// }

impl Stak {
    /// Creates a new Stak structure
    pub fn new() -> Stak {
        let stack: Vec<f64> = Vec::new();
        Stak { stack }
    }

    pub fn print_stack(&self) {
        if self.stack.is_empty() {
            println!("[]");
        } else {
            print!("[");
            for (i, val) in self.stack.iter().enumerate() {
                print!("{}", Number::f64_to_string(*val));
                if i < self.stack.len() - 1 {
                    print!(", ");
                }
            }
            println!("]");
        }
    }

    /// Parses the given token and performs the appropriate action
    /// `token` must be a single token with no leading or trailing whitespace
    /// Returns an error, or the optional top value of the stack
    pub fn parse_token(&mut self, token: &str) -> Result<(), StakError> {
        // Check if the token is a number
        if let Ok(val) = token.parse::<f64>() {
            self.stack.push(val);
        } else {
            // Check compound operators
            if token.starts_with('~') {
                return self.parse_dupe(token);
            }
            if token.starts_with("<<") {
                return self.parse_rotate_left(token);
            }
            if token.starts_with(">>") {
                return self.parse_rotate_right(token);
            }

            // Check standard operators
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
                "<>" => self.swap(), // Swap the top values

                // OPERATORS
                "+" => self.add(),
                "-" => self.sub(),
                "*" => self.mult(),
                "/" => self.div(),
                "^" => self.pow(),
                "**" => self.pow(),
                "%" => self.modulo(),
                "||" => self.parallel(),
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

    /// Parses the index duplication token and executes if valid
    fn parse_dupe(&mut self, token: &str) -> Result<(), StakError> {
        let subtokens: Vec<&str> = token.split('~').collect();
        if subtokens.len() == 2 {
            if subtokens[1].is_empty() {
                self.dupe(0)
            } else if let Ok(i) = subtokens[1].parse::<u64>() {
                self.dupe(i as usize)
            } else {
                Err(InvalidIndex(subtokens[1].to_string()))
            }
        } else {
            Err(InvalidToken(token.to_string()))
        }
    }

    /// Parses the rotate left token and executes if valid
    fn parse_rotate_left(&mut self, token: &str) -> Result<(), StakError> {
        let subtokens: Vec<&str> = token.split("<<").collect();
        if subtokens.len() == 2 {
            if subtokens[1].is_empty() {
                self.rotate_left(1)
            } else if let Ok(i) = subtokens[1].parse::<u64>() {
                self.rotate_left(i as usize)
            } else {
                Err(InvalidIndex(subtokens[1].to_string()))
            }
        } else {
            Err(InvalidToken(token.to_string()))
        }
    }

    /// Parses the rotate right token and executes if valid
    fn parse_rotate_right(&mut self, token: &str) -> Result<(), StakError> {
        let subtokens: Vec<&str> = token.split(">>").collect();
        if subtokens.len() == 2 {
            if subtokens[1].is_empty() {
                self.rotate_right(1)
            } else if let Ok(i) = subtokens[1].parse::<u64>() {
                self.rotate_right(i as usize)
            } else {
                Err(InvalidIndex(subtokens[1].to_string()))
            }
        } else {
            Err(InvalidToken(token.to_string()))
        }
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
        if let Some(a) = self.stack.pop() {
            self.stack.push(a.sqrt());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a log base 2
    fn log2(&mut self) -> Result<(), StakError> {
        if let Some(a) = self.stack.pop() {
            self.stack.push(a.log2());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs an inversion
    fn inv(&mut self) -> Result<(), StakError> {
        if let Some(a) = self.stack.pop() {
            self.stack.push(1_f64 / a);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a floor
    fn floor(&mut self) -> Result<(), StakError> {
        if let Some(a) = self.stack.pop() {
            self.stack.push(a.floor());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a ceiling
    fn ceil(&mut self) -> Result<(), StakError> {
        if let Some(a) = self.stack.pop() {
            self.stack.push(a.ceil());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs an absolute value
    fn abs(&mut self) -> Result<(), StakError> {
        if let Some(a) = self.stack.pop() {
            self.stack.push(a.abs());
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Clones the n-th value of the stack
    fn dupe(&mut self, index: usize) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            if index < self.stack.len() {
                let i = self.stack.len() - 1 - index;
                self.stack.push(self.stack[i]);
                Ok(())
            } else {
                Err(IndexOutOfRange(index))
            }
        } else {
            Err(StackEmpty)
        }
    }

    /// Rotates the stack `i` times to the left
    fn rotate_left(&mut self, i: usize) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            self.stack.rotate_left(i);
        }
        Ok(())
    }

    /// Rotates the stack `i` times to the right
    fn rotate_right(&mut self, i: usize) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            self.stack.rotate_right(i);
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
        if !self.stack.is_empty() {
            let mut sum = 0_f64;
            for x in &self.stack {
                sum += *x;
            }
            self.stack.clear();
            self.stack.push(sum);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a product of all values in the stack
    fn prod(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let mut prod = 1_f64;
            for x in &self.stack {
                prod *= *x;
            }
            self.stack.clear();
            self.stack.push(prod);
            Ok(())
        } else {
            Err(StackEmpty)
        }
    }

    /// Performs a parallel equivalency on all values
    fn parallel(&mut self) -> Result<(), StakError> {
        if !self.stack.is_empty() {
            let mut sum = 0_f64;
            for x in &self.stack {
                sum += 1_f64 / *x;
            }
            self.stack.clear();
            self.stack.push(1_f64 / sum);
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
    #[error("index {0} is out of range")]
    IndexOutOfRange(usize),
    #[error("index `{0}` is invalid")]
    InvalidIndex(String),
    #[error("invalid prefix `{0}`")]
    InvalidPrefix(String),
}

#[cfg(test)]
mod tests {
    use crate::Stak;
    #[test]
    fn push() {
        let mut s = Stak::new();
        s.parse_token("5").unwrap();
        assert_eq!(s.stack, vec![5_f64]);
        assert!(s.parse_token("foo").is_err());
    }
    #[test]
    fn dupe() {
        let mut s = Stak::new();
        s.parse_token("3").unwrap();
        s.parse_token("4").unwrap();
        s.parse_token("5").unwrap();
        s.parse_token("6").unwrap();
        assert_eq!(s.stack, vec![3_f64, 4_f64, 5_f64, 6_f64]);
        s.parse_token("~").unwrap();
        assert_eq!(s.stack, vec![3_f64, 4_f64, 5_f64, 6_f64, 6_f64]);
        s.parse_token("~3").unwrap();
        assert_eq!(s.stack, vec![3_f64, 4_f64, 5_f64, 6_f64, 6_f64, 4_f64]);
    }
}
