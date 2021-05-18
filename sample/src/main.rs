trait Out {
    fn print_twice(&self);
    fn print_add_doller(&self);
}

#[derive(Debug)]
struct Number {
    num: i32
}

impl Out for Number {
    fn print_twice(&self) {
        println!("{}, {}", self.num, self.num);
    }

    fn print_add_doller(&self) {
        println!("$ {}", self.num);
    }
}

fn trait_object(o: &dyn Out) {
    o.print_twice();
    o.print_add_doller();
}

fn main() {
    let num = Number {num: 5};
    
    let out: &dyn Out = &num;
    out.print_twice();
    out.print_add_doller();
    trait_object(out);
}

/*
実行結果:
5, 5
$ 5
5, 5
$ 5
*/