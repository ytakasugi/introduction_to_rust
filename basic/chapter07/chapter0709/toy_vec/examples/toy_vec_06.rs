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

    // このイテレータはミュータブルな要素（Option<&mut i32>）を返す
    let mut iter = v.iter_mut();
    // 最初の要素を8倍する
    iter.next().map(|i| *i *= 8);

    // &mut ToyVec<T>にIntoIteratorを実装し、IterMut<T>を返すようにしたので以下のように使える
    for i in &mut v {
        *i += 10;
    }

    println!("value = {:?}", v);
}