// use std::ops::Add;

fn main() {
    let a:Vec<Vec<i64>> = vec![vec![2, 5],vec![4, 7]];
    let b:Vec<Vec<i64>> = vec![vec![1, 3],vec![6, 9]];

    dot(a.clone(), b.clone());
    // let c = dot(a, b.iter().transpose().collect());

}

fn array(x: Vec<i64>, y: Vec<i64>)->Vec<Vec<i64>> {
    return vec![x, y];
}

fn dot(x: Vec<Vec<i64>>, y: Vec<Vec<i64>>) {
    let transpose_y = transpose(y);
    // println!("{:?}",x);
    // println!("{:?}",y);
    let result = x.iter().map(|val| rowCalc(val.clone(), transpose_y.clone()));
    println!("{:?}",result);
    
    
    fn rowCalc(x: Vec<i64>, y: Vec<Vec<i64>>) {
        let result:Vec<i64> = y
        .iter()
        .map(|y_val| x
                .iter()
                .enumerate()
                .map(|(idx, val)| val * y_val.iter().collect()::<Vec<_>>[idx])
                .sum()
            )
        .collect();
            println!("{:?}",result);
            // return result;
    }

    // fn rowCalc(x: Vec<i64>, y: Vec<Vec<i64>>, num:u32, result:Vec<i64>) {
    //     if num == x.len().try_into().unwrap() {
    //         result
    //     } else {
    //         let sum:i64 = x
    //             .iter()
    //             .enumerate()
    //             .map(|(idx, val)| val * y[idx][num as usize])
    //             .sum();
            
    //         rowCalc(x.clone(), y.clone(), num+1, [result, vec![sum]].concat().into_iter().collect());
    //     }

    // }

}


fn transpose(vec:Vec<Vec<i64>>)->Vec<Vec<i64>> {
    let result = vec[0]
        .iter()
        .enumerate()
        .map(|(idx, _)| vec.iter().map(|v| v[idx]).collect())
        .collect();
    return result;
}

//以下行列入れ替え用 
// trait Transpose<'a, Elem, Iter, T>
// where
//     Elem: 'a,
//     Iter: IntoIterator<Item = &'a Elem>,
//     T: IntoIterator<Item = Iter>,
// {
//     fn transpose(self) -> Transposed<'a, Elem, Iter>;
// }

// impl<'a, Elem, Iter, T> Transpose<'a, Elem, Iter, T> for T
// where
//     Elem: 'a,
//     Iter: IntoIterator<Item = &'a Elem>,
//     T: IntoIterator<Item = Iter>,
// {
//     fn transpose(self) -> Transposed<'a, Elem, Iter> {
//         Transposed {
//             iters: self.into_iter().map(IntoIterator::into_iter).collect(),
//         }
//     }
// }

// struct Transposed<'a, Elem, Iter>
// where
//     Elem: 'a,
//     Iter: IntoIterator<Item = &'a Elem>,
// {
//     iters: Vec<Iter::IntoIter>,
// }

// impl<'a, Elem, Iter> Iterator for Transposed<'a, Elem, Iter>
// where
//     Elem: 'a,
//     Iter: IntoIterator<Item = &'a Elem>,
// {
//     type Item = Vec<&'a Elem>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iters.iter_mut().map(Iterator::next).collect()
//     }
// }
