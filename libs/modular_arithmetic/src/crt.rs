//! A small module allowing computations based on the Chinese Remainder Theorem.

use number_theory::gcd;
use crate::internals::{_mod_inverse, _mod_add, _mod_sub, _mod_mul};

/// A structure representing a CRT-style constraint.
#[derive(PartialEq, Eq, Debug)]
pub struct Constraint {
    /// The residues allowed by this constraint.
    pub residues: Vec<u64>,
    /// The modulus for this constraint.
    pub modulus: u64,
}

impl Constraint {
    /// Create a new `Constraint` with the given residues and modulus.
    pub fn new(residues: Vec<u64>, modulus: u64) -> Constraint {
        Constraint {
            residues: residues,
            modulus: modulus,
        }
    }

    /// Return an empty constraint, not allowing any residues.
    pub fn empty() -> Constraint {
        Constraint {
            residues: Vec::new(),
            modulus: 1,
        }
    }
}

/// Calculates the residue classes that satisfy each of the given constraints.
///
/// Each constraint is in the form of a list of acceptable residue classes, and a modulus.
///
/// # Examples
///
/// ```
/// use modular_arithmetic::{Constraint, crt};
///
/// let con_a = Constraint::new(vec![2], 3);
/// let con_b = Constraint::new(vec![3], 5);
/// let con_c = Constraint::new(vec![2], 7);
///
/// assert_eq!(crt(&vec![con_a, con_b, con_c]), Constraint::new(vec![23], 105));
/// ```
pub fn crt(constraints: &[Constraint]) -> Constraint {

    // Define an inner function that will combine two constraints together.
    fn combine(con_a: &Constraint, con_b: &Constraint) -> Constraint {

        // Find the GCD of the two moduli.
        let (mod_a, mod_b) = (con_a.modulus, con_b.modulus);
        let g = gcd(mod_a, mod_b);
        let (r, s) = (mod_a / g, mod_b / g);

        // To find x == a (mod gr) and x == b (mod gs) we first find k with
        // a + kgr == b (mod gs). This is k = ((b - a) / g) * r_inv (mod s)
        let mut classes = Vec::new();
        let modulus = g * r * s;
        let r_inv = _mod_inverse(r, s);

        if r_inv.is_some() {
            for &res_a in &con_a.residues {
                for &res_b in &con_b.residues {
                    let difference = _mod_sub(res_b, res_a, modulus);
                    if difference % g == 0 {
                        let coeff = _mod_mul(r_inv.unwrap(), r, modulus);
                        let class = _mod_add(res_a, _mod_mul(coeff, difference, modulus), modulus);
                        classes.push(class);
                    }
                }
            }
        }

        Constraint::new(classes, modulus)
    }

    // Combine all the constraints together
    constraints.into_iter().fold(Constraint::new(vec![0], 1), |acc, x| combine(&acc, &x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt() {
        let moduli = vec![3, 5, 7, 11, 13, 17];
        let constraints = moduli
            .iter()
            .map(|&x| Constraint::new(vec![(x - 1) / 2], x))
            .collect::<Vec<_>>();
        let result = crt(&constraints);

        assert_eq!(result, Constraint::new(vec![127627], 255255));

        let moduli = vec![5, 7, 11];
        let constraints = moduli
            .iter()
            .map(|&x| Constraint::new(vec![(x - 1) / 2, (x + 1) / 2], x))
            .collect::<Vec<_>>();
        let mut result = crt(&constraints);
        result.residues.sort();

        assert_eq!(result, Constraint::new(vec![17, 38, 137, 192, 193, 248, 347, 368], 385));
    }
}