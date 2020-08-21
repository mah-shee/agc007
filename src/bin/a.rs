#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        map: [Chars; h],
    }
    let mut ans = true;
    'outer: for i in 0..h {
        if i == h - 1 {
            for j in 0..w {
                if j >= 1 && map[i - 1][j] == '#' && map[i][j - 1] == '#' {
                    ans = false;
                    break 'outer;
                }
            }
            break 'outer;
        }
        for j in 0..w {
            if map[i][j] == '.' {
                continue;
            }
            if j == w - 1 {
                if i >= 1 && map[i][j - 1] == '#' && map[i - 1][j] == '#' {
                    ans = false;
                    break 'outer;
                }
                continue;
            }
            if map[i + 1][j] == '#' && map[i][j + 1] == '#' {
                ans = false;
                break 'outer;
            }
        }
    }
    if ans {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
