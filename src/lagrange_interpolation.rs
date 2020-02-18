use crate::Field;


/// Perform barycentric lagrange interpolation on the slice of points given as an argument
///When x=0 ,the secret=f(0)
pub fn barycentric_interpolate<Fq: Field>(points: &[(Fq, Fq)]) -> Fq {
    let mut result = Fq::zero();
    let mut result_demon = Fq::zero();
    let  _l_x = Fq::one();
    let mut w = Vec::with_capacity(points.len());
    for j in 0..points.len() {
        let x_j = points[j].0;
        w.push(Fq::one());

        for (k, &(x_k, _)) in points.iter().enumerate() {
            if j != k {
                let denom = x_j - x_k;
                w[j] *= Fq::one() / denom;
            }
        }
    }

    for (j, &(x_j, y_j)) in points.iter().enumerate() {
        result += (w[j] * y_j) / -x_j;
        result_demon += w[j]/-x_j;
    }

    result /=  result_demon;

    result
}
