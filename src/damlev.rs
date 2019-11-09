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

    pub fn set1(&mut self, s: &str) -> &mut DamLev {
        self.len1 = cmp::min(s.len(), MAX_CHARS);
        for (i, c) in s.chars().take(self.len1).enumerate() {
            self.chars1[i] = c;
            self.dists[i + 1][0] = i as u8;
        }
        self
    }

    pub fn set2(&mut self, s: &str) -> &mut DamLev {
        self.len2 = cmp::min(s.len(), MAX_CHARS);
        for (j, c) in s.chars().take(self.len2).enumerate() {
            self.chars2[j] = c;
            self.dists[0][j + 1] = j as u8;
        }
        self
    }

    pub fn dist(&mut self) -> u8 {
        let DamLev { chars1, chars2, dists, .. } = self;
        let DamLev { len1, len2, .. } = *self;
        let dist_max = (len1 + len2) as u8;

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
    fn damlev_dist_equality() {
        let mut dl = DamLev::new();
        let sample = [
            "captain",
            "captai",
            "capta",
            "capt",
            "cap",
            "ca",
            "c",
            "",
        ];
        for s in sample.iter() {
            dl.set1(s);
            dl.set2(s);
            assert_eq!(dl.dist(), 0);
        }
    }

    #[test]
    fn damlev_dist_left_prefix() {
        let mut dl = DamLev::new();
        dl.set2("captain");
        let sample = [
            (0, "captain"),
            (1, "captai"),
            (2, "capta"),
            (3, "capt"),
            (4, "cap"),
            (5, "ca"),
            (6, "c"),
            (7, ""),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set1(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_right_prefix() {
        let mut dl = DamLev::new();
        dl.set1("captain");
        let sample = [
            (0, "captain"),
            (1, "captai"),
            (2, "capta"),
            (3, "capt"),
            (4, "cap"),
            (5, "ca"),
            (6, "c"),
            (7, ""),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set2(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_del_continuous() {
        let mut dl = DamLev::new();
        dl.set2("captain");
        let sample = [
            (1, "_captain"),
            (2, "__captain"),
            (3, "___captain"),
            (4, "____captain"),

            (1, "cap_tain"),
            (2, "cap__tain"),
            (3, "cap___tain"),
            (4, "cap____tain"),

            (1, "captain_"),
            (2, "captain__"),
            (3, "captain___"),
            (4, "captain____"),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set1(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_add_continuous() {
        let mut dl = DamLev::new();
        dl.set1("captain");
        let sample = [
            (1, "_captain"),
            (2, "__captain"),
            (3, "___captain"),
            (4, "____captain"),

            (1, "cap_tain"),
            (2, "cap__tain"),
            (3, "cap___tain"),
            (4, "cap____tain"),

            (1, "captain_"),
            (2, "captain__"),
            (3, "captain___"),
            (4, "captain____"),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set2(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_sub_continuous() {
        let mut dl = DamLev::new();
        dl.set1("captain");
        let sample = [
            (1, "_aptain"),
            (2, "__ptain"),
            (3, "___tain"),
            (4, "____ain"),

            (1, "cap_ain"),
            (2, "cap__in"),
            (3, "ca___in"),
            (4, "ca____n"),

            (1, "captai_"),
            (2, "capta__"),
            (3, "capt___"),
            (4, "cap____"),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set2(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_del_intermittent() {
        let mut dl = DamLev::new();
        dl.set2("captain");
        let sample = [
            (1, "_captain"),
            (2, "_c_aptain"),
            (3, "_c_a_ptain"),
            (4, "_c_a_p_tain"),

            (1, "captain_"),
            (2, "captai_n_"),
            (3, "capta_i_n_"),
            (4, "capt_a_i_n_"),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set1(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_add_intermittent() {
        let mut dl = DamLev::new();
        dl.set1("captain");
        let sample = [
            (1, "_captain"),
            (2, "_c_aptain"),
            (3, "_c_a_ptain"),
            (4, "_c_a_p_tain"),

            (1, "captain_"),
            (2, "captai_n_"),
            (3, "capta_i_n_"),
            (4, "capt_a_i_n_"),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set2(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_sub_intermittent() {
        let mut dl = DamLev::new();
        dl.set1("captain");
        let sample = [
            (1, "_aptain"),
            (2, "_a_tain"),
            (3, "_a_t_in"),

            (1, "captai_"),
            (2, "capt_i_"),
            (3, "ca_t_i_"),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set2(s).dist(), *i);
        }
    }

    #[test]
    fn damlev_dist_swp_one() {
        let mut dl = DamLev::new();
        dl.set1("captain");
        let sample = [
            (1, "acptain"),
            (1, "catpain"),
            (1, "captani"),
        ];
        for (i, s) in sample.iter() {
            assert_eq!(dl.set2(s).dist(), *i);
        }
    }
}
