struct XorShift128 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorShift128 {
    fn next(&mut self) -> u32 {
        let mut t = self.w;
        let s = self.x;
        self.w = self.z;
        self.z = self.y;
        self.y = s;

        t ^= t << 11;
        t ^= t >> 8;
        self.x = t ^ s ^ (s >> 19);
        self.x
    }
}

struct NonLinearPrng {
    prng: XorShift128,
}

impl NonLinearPrng {
    fn next_8bits(&mut self) -> u32 {
        let iters = self.prng.next();
        for _ in 0..iters % (1 << 10) {
            self.prng.next();
        }
        self.prng.next() % (1 << 8)
    }

    fn next(&mut self) -> u32 {
        let w0 = self.next_8bits();
        let w1 = self.next_8bits();
        let w2 = self.next_8bits();
        let w3 = self.next_8bits();
        w0 << 24 | w1 << 16 | w2 << 8 | w3
    }
}

fn main() {
    let mut rng = NonLinearPrng {
        prng: XorShift128 {
            x: 0xdeadbeef,
            y: 0x71fe7e55,
            z: 0xb15ec7ed,
            w: 0xdefe47ed,
        },
    };
    for _ in 0..1000 {
        println!("{}", rng.next());
    }
}
