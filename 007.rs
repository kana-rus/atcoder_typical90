use proconio::input;

fn main() {
  input!{
    n: usize,
    mut a: [usize; n],
    q: usize,
    b: [usize; q]
  }

  a.sort();
  
  for student_rate in b {
    match a.binary_search(&student_rate) {
      Ok(_) => { println!("{}", 0); }
      Err(index) => {
        /*
        match index {
          0 => { println!("{} (Err(0))", a[0] - student_rate); }
          n => { println!("{} (Err(n))", student_rate - a[n-1]); }
          _ => {
            println!("{} (Err(_))",
              (a[index] - student_rate)
              .min(student_rate - a[index-1])
            );
          }
        }

        と書きたいが、bynary_search の結果 Err(n) が返ってくるという
        特殊パターンがうまく処理されない (n => {} に any pattern がマッチ
        してその処理が行われてしまう) ため断念
        */
        if index==0 { println!("{}", a[0] - student_rate); }
        else if index==n { println!("{}", student_rate - a[n-1]); }
        else {
          println!("{}",
            (a[index] - student_rate)
            .min(student_rate - a[index-1])        
          );
        }
      }
    }
  }
}
