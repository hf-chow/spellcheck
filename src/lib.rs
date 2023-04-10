pub mod algo; 

#[cfg(test)]
mod tests {
    use crate::algo::algo::osa_dist;

    #[test]
    fn check_osa() {
        assert_eq!(2, osa_dist("DCODE", "DECODER"));
    }
}
