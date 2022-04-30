use proconio::input;
use std::{collections::BinaryHeap, cmp::Reverse};

fn main() {
    /*
    const INF: usize = usize::MAX;
    は 1.43.0 で導入（atcoderのRustは1.42.0）
    */
    const INF: usize = 18446744073709551615; // = usize::MAX (2^64-1)
    input! {
        n: usize, m: usize
    }
    let edge = {
        let mut e: Vec<Vec<(usize,usize)>> = vec![vec![]; 1+n];
        for _ in 0..m {
            input! { a:usize, b:usize, c:usize }
            e[a].push((b, c));
            e[b].push((a, c));
        }
        e
    };
    
    let dijkstra = |start: usize| {
        let mut dist_map = vec![INF; 1+n];
        let mut queue = BinaryHeap::new();
        // (そこまでの距離, 頂点) を要素とする優先度付きキュー
        queue.push((Reverse(0_usize), start));

        while let Some((Reverse(dist), target)) = queue.pop() {
            if dist > dist_map[target] { continue; }
            dist_map[target] = dist;
            for &(next, cost) in &edge[target] {
                if dist_map[next] > dist + cost {
                    dist_map[next] = dist + cost;
                    queue.push((Reverse(dist_map[next]), next));
                }
            }
        }
        dist_map
    };

    let dist_from_1 = dijkstra(1);
    let dist_from_n = dijkstra(n);
    let ans_str = {
        let mut ans = String::new();
        for i in 1..=n {
            ans += &(dist_from_1[i]+dist_from_n[i]).to_string();
            ans += "\n";
        }
        ans
    };
    print!("{}", ans_str);
}
