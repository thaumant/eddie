use std::cmp;



const MAX_CHARS: usize = 15;


#[derive(Debug)]
pub struct DamLev {
    len1: usize,
    len2: usize,
    chars1: [char; MAX_CHARS],
    chars2: [char; MAX_CHARS],
    dists: [[u8; MAX_CHARS + 1]; MAX_CHARS + 1],
}


impl DamLev {
    pub fn new() -> DamLev {
        let len1 = 0;
        let len2 = 0;
        let chars1 = ['0'; MAX_CHARS];
        let chars2 = ['0'; MAX_CHARS];
        let dists = [[0; MAX_CHARS + 1]; MAX_CHARS + 1];
        DamLev { len1, len2, chars1, chars2, dists }
    }

    pub fn dist(&mut self, str1: &str, str2: &str) -> u8 {
        self.len1 = cmp::min(str1.len(), MAX_CHARS);
        self.len2 = cmp::min(str2.len(), MAX_CHARS);

        let DamLev { chars1, chars2, dists, .. } = self;
        let DamLev { len1, len2, .. } = *self;
        let dist_max = cmp::max(len1, len2) as u8;

        for (i, c) in str1.chars().take(len1).enumerate() { chars1[i] = c; }
        for (i, c) in str2.chars().take(len2).enumerate() { chars2[i] = c; }

        for i in 0..len1 + 1 { dists[i][0] = i as u8; }
        for j in 0..len2 + 1 { dists[0][j] = j as u8; }

        for i in 1..len1 + 1 {
            for j in 1..len2 + 1 {
                let cost_sub = (chars1[i - 1] != chars2[j - 1]) as u8;

                let dist_del = dists[i - 1][j] + 1;
                let dist_add = dists[i][j - 1] + 1;
                let dist_sub = dists[i - 1][j - 1] + cost_sub;
                let dist_swp = {
                    let swp =
                        i > 1
                        && j > 1
                        && chars1[i - 1] == chars2[j - 2]
                        && chars2[j - 1] == chars1[i - 2];
                    if swp { dists[i - 2][j - 2] + 1 } else { dist_max }
                };

                let mut dist_min = dist_max;
                dist_min = cmp::min(dist_min, dist_del);
                dist_min = cmp::min(dist_min, dist_add);
                dist_min = cmp::min(dist_min, dist_sub);
                dist_min = cmp::min(dist_min, dist_swp);

                dists[i][j] = dist_min;
            }
        }

        dists[len1][len2]
    }

    pub fn repr(&self) -> String {
        let DamLev { len1, len2, .. } = *self;
        let DamLev { chars1, chars2, dists, .. } = self;

        let mut repr = String::with_capacity((len1 + 2) * (len2 + 2));

        repr.push_str(&format!("{} x {}\n", len1, len2));

        repr.push_str(&"    ");
        for col in 0..len1 {
            repr.push(chars1[col]);
            repr.push(' ');
        }
        repr.push('\n');

        for row in 0..len2 + 1 {
            repr.push(if row == 0 { ' ' } else { chars2[row - 1] });
            repr.push(' ');
            for col in 0..len1 + 1 {
                repr.push_str(&dists[col][row].to_string());
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
    use super::DamLev;

    #[test]
    fn equality() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("", ""), 0);
        assert_eq!(damlev.dist("c", "c"), 0);
        assert_eq!(damlev.dist("ca", "ca"), 0);
        assert_eq!(damlev.dist("cap", "cap"), 0);
        assert_eq!(damlev.dist("capt", "capt"), 0);
        assert_eq!(damlev.dist("capta", "capta"), 0);
        assert_eq!(damlev.dist("captai", "captai"), 0);
        assert_eq!(damlev.dist("captain", "captain"), 0);
    }

    #[test]
    fn left_prefix() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("", "captain"), 7);
        assert_eq!(damlev.dist("c", "captain"), 6);
        assert_eq!(damlev.dist("ca", "captain"), 5);
        assert_eq!(damlev.dist("cap", "captain"), 4);
        assert_eq!(damlev.dist("capt", "captain"), 3);
        assert_eq!(damlev.dist("capta", "captain"), 2);
        assert_eq!(damlev.dist("captai", "captain"), 1);
        assert_eq!(damlev.dist("captain", "captain"), 0);
    }

    #[test]
    fn right_prefix() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("captain", ""), 7);
        assert_eq!(damlev.dist("captain", "c"), 6);
        assert_eq!(damlev.dist("captain", "ca"), 5);
        assert_eq!(damlev.dist("captain", "cap"), 4);
        assert_eq!(damlev.dist("captain", "capt"), 3);
        assert_eq!(damlev.dist("captain", "capta"), 2);
        assert_eq!(damlev.dist("captain", "captai"), 1);
        assert_eq!(damlev.dist("captain", "captain"), 0);
    }

    #[test]
    fn del_continuous() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("_captain", "captain"), 1);
        assert_eq!(damlev.dist("__captain", "captain"), 2);
        assert_eq!(damlev.dist("___captain", "captain"), 3);

        assert_eq!(damlev.dist("cap_tain", "captain"), 1);
        assert_eq!(damlev.dist("cap__tain", "captain"), 2);
        assert_eq!(damlev.dist("cap___tain", "captain"), 3);

        assert_eq!(damlev.dist("captain_", "captain"), 1);
        assert_eq!(damlev.dist("captain__", "captain"), 2);
        assert_eq!(damlev.dist("captain___", "captain"), 3);
    }

    #[test]
    fn add_continuous() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("captain", "_captain"), 1);
        assert_eq!(damlev.dist("captain", "__captain"), 2);
        assert_eq!(damlev.dist("captain", "___captain"), 3);

        assert_eq!(damlev.dist("captain", "cap_tain"), 1);
        assert_eq!(damlev.dist("captain", "cap__tain"), 2);
        assert_eq!(damlev.dist("captain", "cap___tain"), 3);

        assert_eq!(damlev.dist("captain", "captain_"), 1);
        assert_eq!(damlev.dist("captain", "captain__"), 2);
        assert_eq!(damlev.dist("captain", "captain___"), 3);
    }

    #[test]
    fn sub_continuous() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("captain", "_aptain"), 1);
        assert_eq!(damlev.dist("captain", "__ptain"), 2);
        assert_eq!(damlev.dist("captain", "___tain"), 3);

        assert_eq!(damlev.dist("captain", "cap_ain"), 1);
        assert_eq!(damlev.dist("captain", "cap__in"), 2);
        assert_eq!(damlev.dist("captain", "ca___in"), 3);

        assert_eq!(damlev.dist("captain", "captai_"), 1);
        assert_eq!(damlev.dist("captain", "capta__"), 2);
        assert_eq!(damlev.dist("captain", "capt___"), 3);
    }

    #[test]
    fn del_intermittent() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("_captain", "captain"), 1);
        assert_eq!(damlev.dist("_c_aptain", "captain"), 2);
        assert_eq!(damlev.dist("_c_a_ptain", "captain"), 3);

        assert_eq!(damlev.dist("captain_", "captain"), 1);
        assert_eq!(damlev.dist("captai_n_", "captain"), 2);
        assert_eq!(damlev.dist("capta_i_n_", "captain"), 3);
    }

    #[test]
    fn add_intermittent() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("captain", "_captain"), 1);
        assert_eq!(damlev.dist("captain", "_c_aptain"), 2);
        assert_eq!(damlev.dist("captain", "_c_a_ptain"), 3);

        assert_eq!(damlev.dist("captain", "captain_"), 1);
        assert_eq!(damlev.dist("captain", "captai_n_"), 2);
        assert_eq!(damlev.dist("captain", "capta_i_n_"), 3);
    }

    #[test]
    fn sub_intermittent() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("captain", "_aptain"), 1);
        assert_eq!(damlev.dist("captain", "_a_tain"), 2);
        assert_eq!(damlev.dist("captain", "_a_t_in"), 3);

        assert_eq!(damlev.dist("captain", "captai_"), 1);
        assert_eq!(damlev.dist("captain", "capt_i_"), 2);
        assert_eq!(damlev.dist("captain", "ca_t_i_"), 3);
    }

    #[test]
    fn swp_one() {
        let mut damlev = DamLev::new();

        assert_eq!(damlev.dist("captain", "acptain"), 1);
        assert_eq!(damlev.dist("captain", "catpain"), 1);
        assert_eq!(damlev.dist("captain", "captani"), 1);
    }
}
