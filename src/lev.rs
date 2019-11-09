use std::cmp;



const MAX_CHARS: usize = 15;


#[derive(Debug)]
pub struct Lev {
    len1: usize,
    len2: usize,
    chars1: [char; MAX_CHARS],
    chars2: [char; MAX_CHARS],
    dists: [[u8; MAX_CHARS + 1]; MAX_CHARS + 1],
}


impl Lev {
    pub fn new() -> Lev {
        let len1 = 0;
        let len2 = 0;
        let chars1 = ['0'; MAX_CHARS];
        let chars2 = ['0'; MAX_CHARS];
        let dists = [[0; MAX_CHARS + 1]; MAX_CHARS + 1];
        Lev { len1, len2, chars1, chars2, dists }
    }

    pub fn dist(&mut self, str1: &str, str2: &str) -> u8 {
        self.len1 = cmp::min(str1.len(), MAX_CHARS);
        self.len2 = cmp::min(str2.len(), MAX_CHARS);

        for (i, c) in str1.chars().take(self.len1).enumerate() { self.chars1[i] = c; }
        for (i, c) in str2.chars().take(self.len2).enumerate() { self.chars2[i] = c; }

        for i in 0..self.len1 + 1 { self.dists[i][0] = i as u8; }
        for i in 0..self.len2 + 1 { self.dists[0][i] = i as u8; }

        for row in 1..self.len1 + 1 {
            for col in 1..self.len2 + 1 {
                let char1 = self.chars1[row - 1];
                let char2 = self.chars2[col - 1];

                let cost_sub = if char1 == char2 { 0 } else { 1 };

                let dist_del = self.dists[row - 1][col] + 1;
                let dist_add = self.dists[row][col - 1] + 1;
                let dist_sub = self.dists[row - 1][col - 1] + cost_sub;

                let mut dist_min = cmp::max(self.len1, self.len2) as u8;
                dist_min = cmp::min(dist_min, dist_del);
                dist_min = cmp::min(dist_min, dist_add);
                dist_min = cmp::min(dist_min, dist_sub);

                self.dists[row][col] = dist_min;
            }
        }

        self.dists[self.len1][self.len2]
    }

    pub fn repr(&self) -> String {
        let Lev {len1, len2, ..} = *self;
        let mut repr = String::with_capacity((len1 + 2) * (len2 + 2));

        repr.push_str(&format!("{} x {}\n", len1, len2));

        repr.push_str(&"    ");
        for col in 0..len1 {
            repr.push(self.chars1[col]);
            repr.push(' ');
        }
        repr.push('\n');

        for row in 0..len2 + 1 {
            repr.push(if row == 0 { ' ' } else { self.chars2[row - 1] });
            repr.push(' ');
            for col in 0..len1 + 1 {
                repr.push_str(&self.dists[col][row].to_string());
                repr.push(' ');
            }
            repr.push('\n');
        }
        repr.push('\n');

        repr
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
        assert_eq!(lev.dist("capt", "capt"), 0);
        assert_eq!(lev.dist("capta", "capta"), 0);
        assert_eq!(lev.dist("captai", "captai"), 0);
        assert_eq!(lev.dist("captain", "captain"), 0);
    }

    #[test]
    fn left_prefix() {
        let mut lev = Lev::new();
        assert_eq!(lev.dist("", "captain"), 7);
        assert_eq!(lev.dist("c", "captain"), 6);
        assert_eq!(lev.dist("ca", "captain"), 5);
        assert_eq!(lev.dist("cap", "captain"), 4);
        assert_eq!(lev.dist("capt", "captain"), 3);
        assert_eq!(lev.dist("capta", "captain"), 2);
        assert_eq!(lev.dist("captai", "captain"), 1);
        assert_eq!(lev.dist("captain", "captain"), 0);
    }

    #[test]
    fn right_prefix() {
        let mut lev = Lev::new();
        assert_eq!(lev.dist("captain", ""), 7);
        assert_eq!(lev.dist("captain", "c"), 6);
        assert_eq!(lev.dist("captain", "ca"), 5);
        assert_eq!(lev.dist("captain", "cap"), 4);
        assert_eq!(lev.dist("captain", "capt"), 3);
        assert_eq!(lev.dist("captain", "capta"), 2);
        assert_eq!(lev.dist("captain", "captai"), 1);
        assert_eq!(lev.dist("captain", "captain"), 0);
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
