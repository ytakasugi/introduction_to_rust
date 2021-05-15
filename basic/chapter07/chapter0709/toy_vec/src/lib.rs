//use std::fmt;

pub struct ToyVec<T> {
    // `T`型の要素を格納する領域。各要素はヒープ領域に置かれる
    elements: Box<[T]>,
    // `ToyVec`の長さ(要素数)
    len: usize
}

// implブロック内に関連関数やメソッドを定義していく。トレイト境界としてDefaultを設定する
impl<T: Default> ToyVec<T> {
    // `new`は、キャパシティが0の`ToyVec`を作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは指定されたキャパシティを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    // T型の値がsize個格納できるBox<[T]>を返す
    pub fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            // T型のデフォルト値をsize個作り
            .take(size)            
            // Vec<T>に収集してから
            .collect::<Vec<_>>()   
            // Box<[T]>に変換する
            .into_boxed_slice()    
    }

    // ベクタの長さ(要素数)を返す
    pub fn len(&self) -> usize {
        self.len
    }

    // ベクタの現在のキャパシティを返す
    pub fn capacity(&self) -> usize {
        // `elements`の要素数が`ToyVec`のキャパシティになる
        self.elements.len()
    }

    // 第一引数が`&mut self`なので、`ToyVec`の内容を変更する
    // 第二引数が`T`なので、所有権がこのメソッドにムーブする
    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        // 要素を格納する(所有権がムーブする)
        self.elements[self.len] = element;
        self.len += 1;
    }

    // 第一引数が`&self`なので、`ToyVec`は変更されない
    // 第二引数が`usize`なので、値がコピーされる(`usize`にはコピートレイトが実装されている)
    pub fn get(&self, index: usize) -> Option<&T> {
        // もし、`index`が要素数未満であれば
        if index < self.len {
            // `Some`でラップされた不変参照を返す
            Some(&self.elements[index])
        } else {
            // 要素数以上であれば、`None`を返す
            None
        }
    }

    // インデックスが範囲内なら要素への参照を返し、さもなければdefaultで与えた別の値への参照を返す
    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        self.get(index).unwrap_or(default)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            // 要素を1つ削除する
            self.len -= 1;

            // 第二引数で第一引数の要素を置き換え、置き換える前の値を返す
            // `T`型のデフォルト値を与え、もし`T`型が`String`型なら、デフォルト値は空の文字列になる
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            // `elem`を`Some`でラップする
            Some(elem)
        }
    }

    // elementsを拡張する（より大きなサイズで作り直す）
    pub fn grow(&mut self) {
        // もし、現在の`elements`が空なら
        if self.capacity() == 0 {
            // 1要素分の領域を確保する
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保する
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // `self.elements`を置き換える
            // `std::mem::replace`は、第二引数の要素で第一引数の要素を置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // `Vec<T>`の`into_iter(self)`なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        // Iter構造体の定義より、ライフタイムは'vecになる
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }
}


// IntoIteratorトレイトを実装するとfor式での繰り返しができるようになる
impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    // イテレータがイテレートする値の型
    type Item = &'vec T;
    // into_iterメソッドの戻り値の型
    type IntoIter = Iter<'vec, T>;

    // `&ToyVec<T>`に対するトレイト実装なので、`self`の型は`ToyVec<T>`ではなく`&ToyVec<T>`
    // このメソッドは所有権を奪う
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


//
// 要素へのイミュータブルな参照（Option<&T>）を返すイテレータ
//

// ライフタイムの指定により、このイテレータ自身またはnext()で得た&'vec T型の値が
// 生存してる間は、ToyVec<T>は変更できない
pub struct Iter<'vec, T> {
    // `ToyVec`構造体の`elements`を指す不変の参照
    elements: &'vec Box<[T]>,
    // `ToyVec`の長さ
    len: usize,
    // 次に返す要素のインデックス
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関連型（トレイトに関連付いた型）で、このイテレータがイテレートする要素の型を指定する
    type Item = &'vec T;

    // nextメソッドは次の要素を返す
    fn next(&mut self) -> Option<Self::Item> {
        // インデックスが要素数以上の場合、`None`を返す
        if self.pos >= self.len {
            None
        } else {
            // インデックスが要素数未満の場合、不変の参照(`&T`)を`Some`でラップ
            let res = Some(&self.elements[self.pos]);
            // インデックスを1つインクリメントする
            self.pos += 1;
            res
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

