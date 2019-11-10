mod constants;
mod matrix;
mod word;

use std::fmt;
use std::cmp;
use matrix::DistanceMatrix;
use word::Word;


pub struct DamLev {
    word1: Word,
    word2: Word,
    dists: DistanceMatrix,
}


impl DamLev {
    pub fn new() -> DamLev {
        let word1 = Word::new();
        let word2 = Word::new();
        let dists = DistanceMatrix::new();
        DamLev { word1, word2, dists }
    }

    pub fn set1(&mut self, s: &str) -> &mut DamLev {
        self.word1.write(s);
        self
    }

    pub fn set2(&mut self, s: &str) -> &mut DamLev {
        self.word2.write(s);
        self
    }

    pub fn dist(&mut self) -> u8 {
        let DamLev { word1, word2, dists, .. } = self;
        let dist_max = (word1.len + word2.len) as u8;

        for i in 1..word1.len + 1 {
            for j in 1..word2.len + 1 {
                let cost_sub = (word1[i - 1] != word2[j - 1]) as u8;

                let dist_del = dists[(i - 1, j)] + 1;
                let dist_add = dists[(i, j - 1)] + 1;
                let dist_sub = dists[(i - 1, j - 1)] + cost_sub;
                let dist_swp = {
                    let swp =
                        i > 1
                        && j > 1
                        && word1[i - 1] == word2[j - 2]
                        && word2[j - 1] == word1[i - 2];
                    if swp { dists[(i - 2, j - 2)] + 1 } else { dist_max }
                };

                let mut dist_min = dist_max;
                dist_min = cmp::min(dist_min, dist_del);
                dist_min = cmp::min(dist_min, dist_add);
                dist_min = cmp::min(dist_min, dist_sub);
                dist_min = cmp::min(dist_min, dist_swp);

                dists[(i, j)] = dist_min;
            }
        }

        dists[(word1.len, word2.len)]
    }
}


impl fmt::Display for DamLev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} x {}\n", self.word1.len, self.word2.len)?;
        self.fmt_table_head(f)?;
        self.fmt_table_body(f)?;
        Ok(())
    }
}

impl DamLev {
    fn fmt_table_head(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "    ")?;
        for col in 0..self.word1.len {
            write!(f, "{} ", self.word1[col])?;
        }
        write!(f, "\n")?;
        Ok(())
    }

    fn fmt_table_body(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.word2.len + 1 {
            if row == 0 { write!(f, "  ")?; }
            if row >= 1 { write!(f, "{} ", self.word2[row - 1])?; }
            for col in 0..self.word1.len + 1 {
                write!(f, "{} ", self.dists[(col, row)].to_string())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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
    fn damlev_dist_prefix_left() {
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
    fn damlev_dist_prefix_right() {
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
