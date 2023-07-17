use std::{io::{BufWriter, Write, Stdout, stdout}, fs::File};

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
        for _ in 0..iters % 2 {
            self.prng.next();
        }
        self.prng.next() & 1
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
    // for iters in 2u64..=1024 {
    //     for leakage in (1u64..32).rev() {
    //         let state_size = 128;
    //         let gaps = state_size / leakage;
    //         let gap_size = iters;
    //         let security = gaps * gap_size.ilog2() as u64;
    //         if security >= 128 {
    //             let runtime = 32 / leakage * iters;
    //             println!("iters={iters} leakage={leakage} security={security} runtime={runtime}");
    //             break;
    //         }
    //     }
    // }


    let mut rng = NonLinearPrng {
        prng: XorShift128 {
            x: 0xdeadbeef,
            y: 0x71fe7e55,
            z: 0xb15ec7ed,
            w: 0xdefe47ed,
        },
    };
    let stdout = stdout().lock();
    let mut bw = BufWriter::new(stdout);
    for _ in 0.. {
        bw.write_all(rng.next().to_ne_bytes().as_slice()).unwrap();
    }
}
