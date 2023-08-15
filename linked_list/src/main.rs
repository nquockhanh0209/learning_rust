struct Point{
    x: f64,
    y: f64,
}
struct Triangle{
    a: Point,
    b: Point,
    c: Point,
}
impl Triangle{
    fn edge(&self, _a_p: &Point, _b_p: &Point) -> f64{
        f64::sqrt((_a_p.x - _b_p.x).abs()*(_a_p.x - _b_p.x).abs() + (_a_p.y - _b_p.y).abs()*(_a_p.y - _b_p.y).abs())
    }
    fn perimeter(&self) -> f64{
        Self::edge(&self, &self.a, &self.b) + Self::edge(&self, &self.a, &self.c) + Self::edge(&self, &self.c, &self.b)
    }
    fn area(&self) -> f64{
        let p = Self::perimeter(&self)/2.0;
        let a_v = Self::edge(&self, &self.a, &self.b);
        let b_v = Self::edge(&self, &self.b, &self.c);
        let c_v = Self::edge(&self, &self.c, &self.a);
        f64::sqrt(p*(p-a_v)*(p-b_v)*(p-c_v))

    }
}
fn main() {
    let point_a = Point{
        x: 3.0,
        y: 4.0
    };
    let point_b = Point{
        x: 2.0,
        y: 4.0
    };
    let point_c = Point{
        x: 2.5,
        y: 1.0
    };
    let newTri = Triangle{
        a: point_a,
        b: point_b,
        c: point_c,
    };
    println!("triangle area is: {}", newTri.area());
}
