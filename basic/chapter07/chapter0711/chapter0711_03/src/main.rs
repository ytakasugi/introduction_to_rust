use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

// ?演算子を使うためmain関数からResult型を返すようにする
fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["brushwood", "toy-poodle"].iter().cloned().collect();
    // HashSetを可変にするためにRwLockで包み、スレッド間で共有するためにArcで包む
    let dogs = Arc::new(RwLock::new(dogs)); // Arc<RwLock<HashSet<&'static str>>>型

    // PoisonErrorをStringに変換するのに便利な関数を定義しておく
    // 引数`x`は`ToString`を実装している必要がある
    fn stringify(x: impl ToString) -> String {
        x.to_string()
    }

    {
        let ds = dogs.read().map_err(stringify)?;    // readロックを取得する
        assert!(ds.contains("brushwood")); 
        assert!(ds.contains("toy-poodle"));
    }

    // Writeロックを取得しHaseSetに要素を追加する
    dogs.write().map_err(stringify)?.insert("bull-terrier");

    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move ||
        // 別のスレッドでwriteロックを取得しHashSetに要素を追加する
        dogs1.write().map(|mut ds| ds.insert("corgi")).map_err(stringify)
    ).join().expect("Thread error")?;  // スレッドの終了を待つ

    // このスレッドと別スレッドの両方で追加した要素が見える
    assert!(dogs.read().map_err(stringify)?.contains("bull-terrier"));
    assert!(dogs.read().map_err(stringify)?.contains("corgi"));
    Ok(())

}
