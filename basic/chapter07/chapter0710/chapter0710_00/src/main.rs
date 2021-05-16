use std::rc::Rc;

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    // Rcポインタ経由でChild値をヒープ領域に格納する
    let mut rc1 = Rc::new(Child(1));

    // `strong_count`でこの`Child`値の参照カウント(共同所有者の数)が得られる
    // (a) count: 1, rc1: Child(1)
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    {
        // `Rc::clone`で共同所有者を作る。参照カウントが増える
        let rc2 = Rc::clone(&rc1);
        // (b) count: 2, rc1: Child(1). rc2: Child(1)
        println!(
            "(b) count: {}, rc1: {:?}. rc2: {:?}",
            Rc::strong_count(&rc1), rc1, rc2
        );
    }
    // rc2がスコープを抜け、参照カウントが減る
    // (c) count: 1, rc1: Child(1)
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    // 参照カウントが`1`のとき、可変の参照が得られる。そうでないときは`None`が返る
    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    // (d) count: 1, rc1: Child(2)
    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    // Rc::downgradeでWeakポインタが得られる
    // (e) count: 1, rc1: Child(2), weak: (Weak)
    let weak = Rc::downgrade(&rc1);
    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        // 参照カウントは1。Weakポインタはカウントされない
        Rc::strong_count(&rc1),
        rc1,
        weak
    );

    // `Weak`を`Rc`にアップグレードすると、`Child`値にアクセスできる
    if let Some(rc3) = weak.upgrade() {
        // (f) count: 2, rc1: Child(2), rc3: Child(2)
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc3,
        );
    }

    // rc1をドロップする（スコープを抜けたのと同じ） 参照カウントが0になりChildは破棄される
    // Dropping Child(2)
    // (g) count: 0, weak.upgrade(): None
    std::mem::drop(rc1);
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade());
}
