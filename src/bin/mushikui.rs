// ref: https://github.com/drken1215/mushikui_solver

use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
enum Digit {
    Fix(u8),
    Any,
}

impl From<char> for Digit {
    fn from(ch: char) -> Self {
        if ch.is_digit(10) {
            Digit::Fix(ch as u8 - '0' as u8)
        } else if ch == '*' {
            Digit::Any
        } else {
            unreachable!()
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Digit::Fix(d) => {
                write!(f, "{}", d)
            }
            Digit::Any => {
                write!(f, "*")
            }
        }
    }
}

impl Digit {
    fn digit(self) -> Option<u8> {
        match self {
            Digit::Fix(d) => Some(d),
            Digit::Any => None,
        }
    }

    fn accept(self, digit: u8) -> bool {
        match self {
            Digit::Fix(d) => d == digit,
            Digit::Any => true,
        }
    }

    fn is_any(self) -> bool {
        match self {
            Digit::Any => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Mushikui {
    multiplicand: Vec<Digit>,
    multiplier: Vec<Digit>,
    partial_product: Vec<Vec<Digit>>,
    product: Vec<Digit>,
}

impl Mushikui {
    fn new(
        multiplicand: &[char],
        multiplier: &[char],
        partial_product: &[Vec<char>],
        product: &[char],
    ) -> Self {
        assert!(!multiplicand.is_empty());
        assert!(!multiplier.is_empty());
        assert!(multiplicand.len() >= multiplier.len());
        assert_eq!(partial_product.len(), multiplier.len());
        assert!(
            multiplicand.len() + multiplier.len() - 1 <= product.len()
                && product.len() <= multiplicand.len() + multiplier.len()
        );
        assert_ne!(multiplicand[0], '0');
        assert_ne!(multiplier[0], '0');
        for part in partial_product {
            assert_ne!(part[0], '0');
        }
        assert_ne!(product[0], '0');
        let multiplicand = multiplicand
            .iter()
            .copied()
            .map(Digit::from)
            .collect::<Vec<_>>();
        let multiplier = multiplier
            .into_iter()
            .copied()
            .map(Digit::from)
            .collect::<Vec<_>>();
        let partial_product = partial_product
            .into_iter()
            .map(|part| {
                part.into_iter()
                    .copied()
                    .map(Digit::from)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let product = product
            .into_iter()
            .copied()
            .map(Digit::from)
            .collect::<Vec<_>>();
        Self {
            multiplicand,
            multiplier,
            partial_product,
            product,
        }
    }

    fn calculate_partial_product(&self, d: u8) -> Vec<u8> {
        let multiplicand = self
            .multiplicand
            .iter()
            .rev()
            .map_while(|digit| digit.digit())
            .collect::<Vec<_>>();
        let mut prod = Vec::new();
        let mut carry = 0;
        for m in &multiplicand {
            let e = m * d + carry;
            assert!(e <= 90);
            prod.push(e % 10);
            carry = e / 10;
        }
        if carry > 0 {
            assert!(carry <= 9);
            if multiplicand.len() == self.multiplicand.len() {
                prod.push(carry);
            }
        }
        prod.reverse();
        prod
    }

    fn calculate_product(&self) -> Vec<u8> {
        let mut prod = Vec::new();
        let mut carry = 0;
        let partial_product = &self.partial_product;
        for k in 0..(partial_product[partial_product.len() - 1].len() + self.multiplier.len() - 1) {
            let mut s = 0;
            for j in 0..partial_product.len() {
                if k >= j && k - j < partial_product[j].len() {
                    let d = partial_product[j][partial_product[j].len() - (k - j) - 1]
                        .digit()
                        .unwrap_or(0);
                    s += u32::from(d);
                }
            }
            prod.push(((s + carry) % 10) as u8);
            carry = (s + carry) / 10;
        }
        while carry > 0 {
            prod.push((carry % 10) as u8);
            carry /= 10;
        }
        prod.reverse();
        prod
    }

    fn rec_multiplicand(&mut self, i: usize, result: &mut Vec<Self>) {
        let len = self.multiplicand.len();
        if i == len {
            self.rec_multiplier(i, 0, true, result);
            return;
        }

        for d in 0..=9 {
            if d == 0 && i == len - 1 {
                continue;
            }
            let old = self.multiplicand[len - i - 1];
            if old.accept(d) {
                self.multiplicand[len - i - 1] = Digit::Fix(d);
                self.rec_multiplier(i, 0, false, result);
                self.multiplicand[len - i - 1] = old;
            }
        }
    }

    fn rec_multiplier(&mut self, i: usize, j: usize, last: bool, result: &mut Vec<Self>) {
        let len = self.multiplier.len();
        if j == len {
            if last {
                assert_eq!(i, self.multiplicand.len());
                for d in &self.multiplicand {
                    assert!(d.digit().is_some());
                }
                for d in &self.multiplier {
                    assert!(d.digit().is_some());
                }
                let old_partial_product = self.partial_product.clone();
                let mut ok = true;
                for j in 0..self.multiplier.len() {
                    let d = self.multiplier[self.multiplier.len() - j - 1]
                        .digit()
                        .unwrap();
                    let partial_product = self
                        .calculate_partial_product(d)
                        .into_iter()
                        .collect::<Vec<_>>();
                    let accept = partial_product
                        .iter()
                        .zip(self.partial_product[j].iter())
                        .all(|(&d, e)| e.accept(d));
                    if self.partial_product[j].len() == partial_product.len() && accept {
                        self.partial_product[j] =
                            partial_product.into_iter().map(Digit::Fix).collect();
                    } else {
                        ok = false;
                    }
                }
                if ok {
                    let product = self.calculate_product();
                    let accept = product
                        .iter()
                        .zip(self.product.iter())
                        .all(|(&d, e)| e.accept(d));
                    if product.len() == self.product.len() && accept {
                        let old_product = self.product.clone();
                        self.product = product.into_iter().map(Digit::Fix).collect();
                        result.push(Clone::clone(self));
                        self.product = old_product;
                    }
                }
                self.partial_product = old_partial_product;
            } else {
                assert!(i < self.multiplicand.len());
                self.rec_multiplicand(i + 1, result);
            }
            return;
        }

        let part = &self.partial_product[j];
        if !last && part[part.len() - i - 1].is_any() {
            self.rec_multiplier(i, j + 1, last, result);
            return;
        }

        for d in 1..=9 {
            let old_digit = self.multiplier[len - j - 1];
            if old_digit.accept(d) {
                let old_part = self.partial_product[j].clone();
                let part = self.calculate_partial_product(d);
                let accept = part
                    .iter()
                    .rev()
                    .zip(old_part.iter().rev())
                    .all(|(&d, e)| e.accept(d));
                if part.len() <= old_part.len() && accept {
                    self.multiplier[len - j - 1] = Digit::Fix(d);
                    for k in 0..(part.len().min(old_part.len())) {
                        self.partial_product[j][old_part.len() - k - 1] =
                            Digit::Fix(part[part.len() - k - 1]);
                    }
                    self.rec_multiplier(i, j + 1, last, result);
                    self.multiplier[len - j - 1] = old_digit;
                    self.partial_product[j] = old_part;
                }
            }
        }
    }

    fn solve(&mut self) -> Vec<Self> {
        let mut result = Vec::new();
        self.rec_multiplicand(0, &mut result);
        result
    }
}

impl Display for Mushikui {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let width = self.product.len();
        let multiplicand = self
            .multiplicand
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>();
        writeln!(
            f,
            "{digits:>width$}",
            digits = multiplicand.join(""),
            width = width
        )?;
        let multiplier = self
            .multiplier
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>();
        writeln!(
            f,
            "{digits:>width$}",
            digits = multiplier.join(""),
            width = width
        )?;
        writeln!(f, "{}", "-".repeat(width))?;
        for (i, part) in self.partial_product.iter().enumerate() {
            let part = part.iter().map(|d| d.to_string()).collect::<Vec<_>>();
            writeln!(
                f,
                "{digits:>width$}",
                digits = part.join(""),
                width = width - i
            )?;
        }
        writeln!(f, "{}", "-".repeat(width))?;
        let product = self
            .product
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>();
        write!(
            f,
            "{digits:>width$}",
            digits = product.join(""),
            width = width
        )
    }
}

fn mushikui_from(s: &str) -> Mushikui {
    let lines = s
        .trim()
        .lines()
        .filter(|s| !s.contains("---"))
        .map(|s| s.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();
    let n = lines.len();
    assert!(n >= 4);
    Mushikui::new(&lines[0], &lines[1], &lines[2..(n - 1)], &lines[n - 1])
}

fn main() {
    let problems = [
        // Q.1
        r#"
          9
          *
        ---
         27
        ---
         27
        "#,
        // Q.2
        r#"
         27
          *
        ---
        **9
        ---
        **9
        "#,
        // Q.6
        r#"
          *1
          2*
        ----
         **3
        *4*
        ----
        ****
        "#,
        // Q.7
        r#"
         2*
         4*
        ---
         6*
        *8
        ---
        ***
        "#,
        r#"
         7*
         **
        ---
        *5*
        **
        ---
        *3*
        "#,
        // Q.15
        r#"
            *1**
            2***
        -------
           *3**
         **4**
        ****5
        ***6
        -------
        ****7**
        "#,
        // Q.17
        r#"
              *1*****
               ******
        -------------
              2*3****
            ********
           **4*5*6*
           *******
          ****7*8
        ********
        -------------
        *******9*****
        "#,
        // Q.22
        r#"
                            ************************
                                ********************
        --------------------------------------------
                           *********************9*0*
                          ********************8*1**
                          ******************7*2***
                        ******************6*3****
                       *****************5*4*****
                       ***************4*5******
                     ***************3*6*******
                     *************2*7********
                   *************1*8*********
                   ***********0*9**********
                 ***********9*0***********
                **********8*1************
                ********7*2*************
               *******6*3**************
              ******5*4***************
             *****4*5****************
            ****3*6*****************
          ****2*7******************
          **1*8*******************
        **0*9********************
        --------------------------------------------
        ********************************************
    "#,
    ];

    for problem in problems {
        let mut mushikui = mushikui_from(problem);
        println!("{}", mushikui);
        println!();
        let result = mushikui.solve();
        assert_eq!(result.len(), 1);
        println!("{}", result[0]);
    }
}
