use crate::ff_gf256::GF256;
use crate::lagrange_interpolation::barycentric_interpolate;
use crate::polynomial::Polynomial;
use crate::Field;
use rand::{thread_rng, Rng};
use subtle::ConstantTimeEq;

/// Represents a set of points which make up a share of a secret
pub struct Share<Fq: Field> {
    points: Vec<(Fq, Fq)>,
}

/// A set of functions for Shamir Secret Sharing
pub struct Shamir;

impl Shamir {
    /// separate a secret (a byte slice) into n shares, of which only k are needed to recover the secret
    /// This requires 0 < k <= n < 256 and supports arbitrary sized secrets
    pub fn separate(secret: &[u8], k: usize, n: usize) -> Result<Vec<Share<GF256>>, &'static str> {
        if n == 0 || n >= 255 {
            return Err("n must be between 1 and 255");
        } else if k == 0 || k > n {
            return Err("k must be between 1 and n");
        }

        let mut shares: Vec<Share<GF256>> = Vec::with_capacity(n);
        for _ in 0..n {
            shares.push(Share {
                points: Vec::with_capacity(secret.len()),
            });
        }

        let mut point_set = std::collections::HashSet::new();
        for &secret_byte in secret {
            let polynomial: Polynomial<GF256> =
                Polynomial::from_intercept(GF256(secret_byte), k - 1);
            for share in shares.iter_mut() {
                let mut random_byte = thread_rng().gen::<u8>();
                while point_set.contains(&random_byte) || random_byte.ct_eq(&0).into() {
                    random_byte = thread_rng().gen::<u8>();
                }
                // Need to verify no random byte is repeated
                point_set.insert(random_byte);

                let x = GF256(random_byte);
                let y = polynomial.evaluate_at(x);
                share.points.push((x, y));
            }
            point_set.clear();
        }

        Ok(shares)
    }


    /// Combines a slice of shares to recover the secret which is returned as a vector of bytes
    pub fn recover(shares: &[Share<GF256>]) -> Vec<u8> {
        let secret_size = shares.first().unwrap().points.len();
        let mut result = Vec::with_capacity(secret_size);

        for i in 0..secret_size {
            let points = shares
                .iter()
                .map(|share| share.points[i])
                .collect::<Vec<(GF256, GF256)>>();
            let secret_byte = barycentric_interpolate(&points);
            result.push(secret_byte.0);
        }

        result
    }
}


/***
#[cfg(test)]

mod test {
    use crate::shamir_secret::Shamir;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_separate_and_recover_12_of_18() {
        test_separate_and_recover(27, 12, 18);
    }



    fn test_separate_and_recover(size: usize, k: usize, n: usize) {
        let mut random_secret = Vec::with_capacity(size);
        (0..size).for_each(|_| random_secret.push(thread_rng().gen::<u8>()));

        let shares = Shamir::separate(&random_secret, k, n).unwrap();
        let combined = Shamir::recover(&shares);

        assert_eq!(combined, random_secret);
    }

 
}
***/

