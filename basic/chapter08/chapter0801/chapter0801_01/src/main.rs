// デカルト座標
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

// 座標
struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    // 関連関数
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

// `CartesianCoord`構造体に`Coordinates`トレイトを実装する
impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

// `PolarCoord`構造体に`Coordinates`トレイトを実装する
impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

// タプルにもトレイトを実装できる
impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn main() {
    // 値を用意する
    let point = (1.0, 1.0);

    // トレイトのメソッドを呼ぶ
    let c = point.to_cartesian();
    println!("x = {}, y = {}", c.x, c.y);

    // 同じくトレイトの関連関数を呼ぶ(後述
    let p: PolarCoord = Coordinates::from_cartesian(c);
    println!("r = {}, θ = {}", p.r, p.theta);
}
