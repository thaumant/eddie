use std::cmp;

const MAX_SIZE: usize = 16;


pub struct Lev {
    dists: [[u8; MAX_SIZE + 1]; MAX_SIZE + 1],
}


impl Lev {
    pub fn new() -> Lev {
        let dists = [[0; MAX_SIZE + 1]; MAX_SIZE + 1];
        Lev { dists }
    }

    pub fn dist<'a>(&mut self, str1: &'a str, str2: &'a str) -> u8 {
        let i1_max = cmp::min(str1.len(), MAX_SIZE);
        let i2_max = cmp::min(str2.len(), MAX_SIZE);

        for i1 in 0..i1_max + 1 { self.dists[i1][0] = i1 as u8; }
        for i2 in 0..i2_max + 1 { self.dists[0][i2] = i2 as u8; }

        for (_i1, char1) in str1.chars().take(i1_max).enumerate() {
            for (_i2, char2) in str2.chars().take(i2_max).enumerate() {
                let i1 = _i1 + 1;
                let i2 = _i2 + 1;

                let cost_sub = if char1 == char2 { 0 } else { 1 };

                let dist_del = self.dists[i1 - 1][i2] + 1;
                let dist_add = self.dists[i1][i2 - 1] + 1;
                let dist_sub = self.dists[i1 - 1][i2 - 1] + cost_sub;

                let mut dist_min = 3;
                dist_min = cmp::min(dist_min, dist_del);
                dist_min = cmp::min(dist_min, dist_add);
                dist_min = cmp::min(dist_min, dist_sub);

                self.dists[i1][i2] = dist_min;
            }
        }

        self.dists[i1_max][i2_max]
    }
}



#[cfg(test)]
mod tests {
    use super::Lev;

    #[test]
    fn equality() {
        let mut lev = Lev::new();
        assert_eq!(lev.dist("", ""), 0);
        assert_eq!(lev.dist("c", "c"), 0);
        assert_eq!(lev.dist("ca", "ca"), 0);
        assert_eq!(lev.dist("cap", "cap"), 0);
    }

    #[test]
    fn left_empty() {
        let mut lev = Lev::new();
        assert_eq!(lev.dist("", "c"), 1);
        assert_eq!(lev.dist("", "ca"), 2);
        assert_eq!(lev.dist("", "cap"), 3);
    }

    #[test]
    fn right_empty() {
        let mut lev = Lev::new();
        assert_eq!(lev.dist("c", ""), 1);
        assert_eq!(lev.dist("ca", ""), 2);
        assert_eq!(lev.dist("cap", ""), 3);
    }

    #[test]
    fn del_continuous() {
        let mut lev = Lev::new();

        assert_eq!(lev.dist("_captain", "captain"), 1);
        assert_eq!(lev.dist("__captain", "captain"), 2);
        assert_eq!(lev.dist("___captain", "captain"), 3);

        assert_eq!(lev.dist("cap_tain", "captain"), 1);
        assert_eq!(lev.dist("cap__tain", "captain"), 2);
        assert_eq!(lev.dist("cap___tain", "captain"), 3);

        assert_eq!(lev.dist("captain_", "captain"), 1);
        assert_eq!(lev.dist("captain__", "captain"), 2);
        assert_eq!(lev.dist("captain___", "captain"), 3);
    }

    #[test]
    fn add_continuous() {
        let mut lev = Lev::new();

        assert_eq!(lev.dist("captain", "_captain"), 1);
        assert_eq!(lev.dist("captain", "__captain"), 2);
        assert_eq!(lev.dist("captain", "___captain"), 3);

        assert_eq!(lev.dist("captain", "cap_tain"), 1);
        assert_eq!(lev.dist("captain", "cap__tain"), 2);
        assert_eq!(lev.dist("captain", "cap___tain"), 3);

        assert_eq!(lev.dist("captain", "captain_"), 1);
        assert_eq!(lev.dist("captain", "captain__"), 2);
        assert_eq!(lev.dist("captain", "captain___"), 3);
    }

    #[test]
    fn sub_continuous() {
        let mut lev = Lev::new();

        assert_eq!(lev.dist("captain", "_aptain"), 1);
        assert_eq!(lev.dist("captain", "__ptain"), 2);
        assert_eq!(lev.dist("captain", "___tain"), 3);

        assert_eq!(lev.dist("captain", "cap_ain"), 1);
        assert_eq!(lev.dist("captain", "cap__in"), 2);
        assert_eq!(lev.dist("captain", "ca___in"), 3);

        assert_eq!(lev.dist("captain", "captai_"), 1);
        assert_eq!(lev.dist("captain", "capta__"), 2);
        assert_eq!(lev.dist("captain", "capt___"), 3);
    }

    #[test]
    fn del_intermittent() {
        let mut lev = Lev::new();

        assert_eq!(lev.dist("_captain", "captain"), 1);
        assert_eq!(lev.dist("_c_aptain", "captain"), 2);
        assert_eq!(lev.dist("_c_a_ptain", "captain"), 3);

        assert_eq!(lev.dist("captain_", "captain"), 1);
        assert_eq!(lev.dist("captai_n_", "captain"), 2);
        assert_eq!(lev.dist("capta_i_n_", "captain"), 3);
    }

    #[test]
    fn add_intermittent() {
        let mut lev = Lev::new();

        assert_eq!(lev.dist("captain", "_captain"), 1);
        assert_eq!(lev.dist("captain", "_c_aptain"), 2);
        assert_eq!(lev.dist("captain", "_c_a_ptain"), 3);

        assert_eq!(lev.dist("captain", "captain_"), 1);
        assert_eq!(lev.dist("captain", "captai_n_"), 2);
        assert_eq!(lev.dist("captain", "capta_i_n_"), 3);
    }

    #[test]
    fn sub_intermittent() {
        let mut lev = Lev::new();

        assert_eq!(lev.dist("captain", "_aptain"), 1);
        assert_eq!(lev.dist("captain", "_a_tain"), 2);
        assert_eq!(lev.dist("captain", "_a_t_in"), 3);

        assert_eq!(lev.dist("captain", "captai_"), 1);
        assert_eq!(lev.dist("captain", "capt_i_"), 2);
        assert_eq!(lev.dist("captain", "ca_t_i_"), 3);
    }
}
