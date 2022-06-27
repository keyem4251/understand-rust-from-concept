pub fn fn_5_4() {
    // 関連型
    // 独立な型パラメータと従属する型パラメータをトレイトに組み込む
}

// TとSが同じサイズ(i32, u32)
// Tがi32のときにSをf64にするような意図しない実装を禁止できない
// trait IAbs<T, S> {
//     fn iabs(self) -> S;
// }

// impl IAbs<i32, u32> for i32 {
//     fn iabs(self) -> u32 {
//         if self >= 0 {
//             self as u32
//         } else {
//             -self as u32
//         }
//     }
// }


trait IAbs {
    type Output;
    fn iabs(self) -> <Self as IAbs>::Output;
}

impl IAbs for i32 {
    type Output = u32;
    fn iabs(self) -> <Self as IAbs>::Output {
        if self >= 0 {
            self as <Self as IAbs>::Output
        } else {
            (-self) as <Self as IAbs>::Output
        }
    }
}
