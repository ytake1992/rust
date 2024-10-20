fn main() {
    let a:Vec<Vec<i64>> = vec![vec![2, 5],vec![4, 7]];
    let b:Vec<Vec<i64>> = vec![vec![1, 3],vec![6, 9]];

    // let c = dot(a, b.iter().transpose().collect());

    // a.iter().for_each(|x|println!("{:?}",x.iter().enumerate()));

    println!("{:?}",a.iter().enumerate().collect::<Vec<_>>());
}

fn array(x: Vec<i64>, y: Vec<i64>)->Vec<Vec<i64>> {
    return vec![x, y];
}

// fn dot(x: Vec<Vec<i64>>, y: Vec<Vec<i64>>) {
//     return rowCalc(x[0], y[0]);
    
//     fn rowCalc(x: Vec<i64>, y: Vec<i64>, num:<i64>) -> Vec<i64> {
//         let a = x.iter().enumerate().map(|val| val.0 * y[*val.1 as usize]);
//         return vec![0,0];
//     }
// }

//以下行列入れ替え用 
trait Transpose<'a, Elem, Iter, T>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
    T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter>;
}

impl<'a, Elem, Iter, T> Transpose<'a, Elem, Iter, T> for T
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
    T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter> {
        Transposed {
            iters: self.into_iter().map(IntoIterator::into_iter).collect(),
        }
    }
}

struct Transposed<'a, Elem, Iter>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
{
    iters: Vec<Iter::IntoIter>,
}

impl<'a, Elem, Iter> Iterator for Transposed<'a, Elem, Iter>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
{
    type Item = Vec<&'a Elem>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iters.iter_mut().map(Iterator::next).collect()
    }
}
