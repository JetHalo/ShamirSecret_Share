use crate::Field;


/// Represents a polynomial with coefficients in Fq
#[derive(PartialEq, Debug)]
pub struct Polynomial<Fq: Field> {
    coefficients: Vec<Fq>,
}



impl<Fq: Field> Polynomial<Fq> {
    /// Generates a polynomial with random coefficients and a given intercept
    pub fn from_intercept(intercept: Fq, degree: usize) -> Polynomial<Fq> {
        let mut result = Vec::with_capacity(degree);
        let  fq = Fq::gen_random();
        for _ in 1..degree {
            result.push(fq);
        }
        result.push(intercept);

        Polynomial {
            coefficients: result
        }
    }

    /// Evaluates a Polynomial with coefficients in field Fq at a given x value
    pub fn evaluate_at(&self, x: Fq) -> Fq {

        self.coefficients
            .iter()
            .skip(1)
            .fold(*self.coefficients.first().unwrap(), |result, &a_i| {
                a_i + (result * x)
            })
    }
}
