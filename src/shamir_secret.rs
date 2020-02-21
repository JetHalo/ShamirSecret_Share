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
    /// 
    /// 
    /// We define a function that separare the secret into n shares ,secret is its length and its a u8 bytes slice .
    pub fn separate(secret: &[u8], k: usize, n: usize) -> Result<Vec<Share<GF256>>, &'static str> {
      // This block to make sure the the range of k and n is correct,because the definition of 
      //Shamir secret is  a secret which seprate into n shares ,and k of the n shares (k<n)could recover
      // the secret and at the the same time we define this n shares size between 1 to 255.
        if n == 0 || n >= 255 {
            return Err("n must be between 1 and 255");
        } else if k == 0 || k > n {
            return Err("k must be between 1 and n");
        }
        // Here define the shares in a vector which have an empty heap but has n capacity.And the type
        //of the shares are Galois field 2^8.
        let mut shares: Vec<Share<GF256>> = Vec::with_capacity(n);
        //make every share contains the points which the amount of it is same as the secret length.
        for _ in 0..n {
            shares.push(Share {
                points: Vec::with_capacity(secret.len()),
            });
        }
         // define a  Hashset collection  name point_set ，this point set help us to make sure the
         //random byte is not repeated.
        let mut point_set = std::collections::HashSet::new();
        // this progress use the methods from_intercept in mod polynomial  to gennerate the coeffients 
        // of the Polynomial.and its a for loop to generate each points of every share in n shares.
        for &secret_byte in secret {
            //we construct the polynomial that is k degree function which has a intercept made by the element of the secret.
            //this fuction return the coeffients and the intercept
            let polynomial: Polynomial<GF256> =
                Polynomial::from_intercept(GF256(secret_byte), k - 1);
              //  to verify no random byte is repeated  
            for share in shares.iter_mut() {
                // genarate byte randomly 
                let mut random_byte = thread_rng().gen::<u8>();
                //if point_set contains the random_byte already or it generate o,we should regenarate the random_byte
                while point_set.contains(&random_byte) || random_byte.ct_eq(&0).into() {
                    random_byte = thread_rng().gen::<u8>();
                }
                //we insert the random_byte we had genarated into the point_set which is the type of hashset.
                point_set.insert(random_byte);
                 //put the random_byte into struct GF256 to genarate the  horizontal axis of the points.
                 //because this is a for loop of shares ,so end of this loop,it will generate n x for the each poins of n shares first.
                let x = GF256(random_byte);
                // use the funcion evaluate to caculate the vertical axis of the points in the random genarate polymonimal
                let y = polynomial.evaluate_at(x);
                //push the points into the shares collection.
                //then they became the firt element of the share ,when the first time of the for loop ended it will 
                // generate the first element of each of the n shares.
                share.points.push((x, y));
            }
            //clear the point_set 
            point_set.clear();
        }
        //return the shares
        Ok(shares)
    }


    /// Combines a slice of shares to recover the secret which is returned as a vector of bytes
    pub fn recover(shares: &[Share<GF256>]) -> Vec<u8> {
        // define the length of the generated points we will use to combine to.
        let secret_size = shares.first().unwrap().points.len();
        //define the capacity of the result which is recoverd by the methods.
        let mut result = Vec::with_capacity(secret_size);
         //we map the shares and rebuild the construct such as
         // {Element0[<point1<x,y>,point2<x,y>..pointn<x,y>]...Elementsize[<point1<x,y>,point2<x,y>..pointn<x,y>]}
        for i in 0..secret_size {
            let points = shares
                .iter()
                .map(|share| share.points[i])
                .collect::<Vec<(GF256, GF256)>>();
        //use the  x value of these points and barycentric lagrange interpolation to recover the origin each bytes of the secret.
            let secret_byte = barycentric_interpolate(&points);
            result.push(secret_byte.0);
        }
        //return the secret.
        result
    }
}


//Explain 　the librarise:
//
//use crate::ff_gf256::GF256;
//This is a galois field .When we deal with the crptography problem we should make sure we are in a field. 
//In a galois field all elements are their own additive inverse,all elements except zero have inverses,
//all elements generate themselves.
// 2 is prime number，so we genarate polynomial can satisfied the galois field 
//ff_gf256 is a module for field elements over the field GF(2^8) with irreducible polynomial x^8+x^4+x^3+x+1


//use crate::lagrange_interpolation::barycentric_interpolate;
//when we want to get a curve of n-1，we need n points.
//we can derive a formula，but we dont show the formula here.
// The formula shows when we give the points  we could find a method to calulate the f(x).
// As we make the X=0 ,we get f(x)=0+intercpept.
//so when we know the value of f(x) at x=0,then the intercept equal f(x) and the interpept is the key .
//because we construct the polynomial with the K points of the n points,they are in the same curve,
// we calulate the valuve of the f(x) through those k points.


//use crate::polynomial::Polynomial;
//we use this lib to generate polynomial . Finally ,we want k shares to recover the secret  ,it seems like
//we use k ponits to recover the polymonial which has k-1 degree.

//use crate::Field;
//This is  a traitbound of Field.  it represents an element of a field.

//use rand::{thread_rng, Rng};
//This lib helps me to generate random number

//use subtle::ConstantTimeEq;
//This lib help me to transform the type bool to number [0 or 1]
//
//
//




#[cfg(test)]

mod test {
    use crate::shamir_secret::Shamir;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_separate_and_recover_12_of_18() {
        test_separate_and_recover(27, 12, 18);
    }
    #[test]
    fn test_separate_and_recover_2_of_2() {
        test_separate_and_recover(32, 2, 2);
    }
    #[test]
    fn test_separate_and_recover_12_of_22() {
        test_separate_and_recover(32, 12, 22);
    }
    #[test]
    fn test_separate_and_recover_insufficient() {
        test_separate_and_combine_insufficient_shares(28, 8, 15);
    }
    
   

    fn test_separate_and_recover(size: usize, k: usize, n: usize) {
        let mut random_secret = Vec::with_capacity(size);
        (0..size).for_each(|_| random_secret.push(thread_rng().gen::<u8>()));

        let shares = Shamir::separate(&random_secret, k, n).unwrap();
        let combined = Shamir::recover(&shares);

        assert_eq!(combined, random_secret);
    }

    fn test_separate_and_combine_insufficient_shares(size: usize, k: usize, n: usize) {
        let mut random_secret = Vec::with_capacity(size);
        (0..size).for_each(|_| random_secret.push(thread_rng().gen::<u8>()));

        let shares = Shamir::separate(&random_secret, k, n).unwrap();
        let combined = Shamir::recover(&shares[0..k - 2]);

        assert_ne!(combined, random_secret);
    }
 
}

