#![allow(dead_code, unused_variables)]
#![feature(label_break_value)]

fn uninit() {
    // let x: f32;
    let x: f32 = 0.0;
    let y = 1.0;
    let z = x + y;
}

#[derive(Default, Clone)]
struct Point {
    x: f32,
}

fn move_() {
    let x = Point::default();
    let y = Point::default();
    let z = y.clone();
    let c = x.x + y.x;
}

fn mut_ref_access() {
    let mut v = vec![0, 1, 2];
    // for item in &v {
    //     if *item > 0 {
    //         v.remove(0);
    //     }
    // }
}

// fn lifetimes() {
//     let mut x: &f32;
//
//     'a: {
//         let y = 0.0;
//         x = &y;
//     } // y is droped, x is dangling
//
//     println!("{}", *x);
// }

struct Buffer<'a> {
    samples: &'a [f32],
}

impl<'a> Buffer<'a> {
    fn process(&self) {
        for sample in self.samples {
            println!("{}", sample);
        }
    }
}

struct User {
    name: &'static str,
}

fn main() {
    let user = User {
        name: "tom morello",
    };

    const NUM_SAMPLES: u16 = 128;
    const BUFFER_SIZE: u16 = 8;
    const NUM_BUFFERS: u16 = NUM_SAMPLES / BUFFER_SIZE;

    'a: {
        let samples: Vec<_> = (0..NUM_SAMPLES).map(f32::from).collect();

        for buf_idx in 0..NUM_BUFFERS {
            let start_sample_idx = (buf_idx * BUFFER_SIZE) as usize;
            let last_sample_idx = ((buf_idx + 1) * BUFFER_SIZE) as usize;

            'b: {
                Buffer {
                    samples: &samples[start_sample_idx..last_sample_idx],
                }
                .process();
            }
        }
    }

    //re-use audio_buffer here, no no no
}
