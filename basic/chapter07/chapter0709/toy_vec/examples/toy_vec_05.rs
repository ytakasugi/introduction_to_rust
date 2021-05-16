use toy_vec::ToyVec;

fn main() {
    // `ToyVec`を作成
    let mut v = ToyVec::new();
    // 要素を追加
    v.push(1);
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(5);

    // 各要素を合計した値を格納する`sum`を宣言
    let mut sum = 0;

    // &ToyVec<T>にIntoIteratorを実装し、Iter<T>を返すようにしたので以下のように使える
    for i in &v {
        sum += *i;
    }

    println!("sum = {}", sum);
}