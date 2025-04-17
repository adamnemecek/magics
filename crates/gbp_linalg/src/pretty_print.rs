#![allow(
    clippy::module_name_repetitions,
    clippy::uninlined_format_args,
    clippy::used_underscore_items
)]
//! Pretty printing of matrices and vectors. Useful for debugging and
//! visualizing the contents of a matrix or vector.

use super::prelude::*;

// const CELL_WIDTH: usize = 14;
const PRECISION: usize = 3;

const RESET_TEXT: &str = "\x1b[0m";
const RED_TEXT: &str = "\x1b[31m";
const GREEN_TEXT: &str = "\x1b[32m";
// const BLUE_TEXT: &str = "\x1b[34m";
const YELLOW_TEXT: &str = "\x1b[33m";
const MAGENTA_TEXT: &str = "\x1b[35m";
const CYAN_TEXT: &str = "\x1b[36m";
// const BOLD_TEXT: &str = "\x1b[1m";
// const UNDERLINE_TEXT: &str = "\x1b[4m";
// const ITALIC_TEXT: &str = "\x1b[3m";

const BAR: char = '│';
const UPPER_LEFT_CORNER: char = '╭';
const UPPER_RIGHT_CORNER: char = '╮';
const LOWER_LEFT_CORNER: char = '╰';
const LOWER_RIGHT_CORNER: char = '╯';
// const HORIZONTAL_LINE: char = '─';

/// Count the number of integral digits in a floating point number.
/// Useful for determining the width of the cell when pretty printing a matrix.
/// ```
/// use gbp_linalg::pretty_print::num_of_integral_digits;
/// assert_eq!(num_of_integral_digits(0.0), Some(1));
/// assert_eq!(num_of_integral_digits(1.0), Some(1));
/// assert_eq!(num_of_integral_digits(10.0), Some(2));
/// assert_eq!(num_of_integral_digits(100.0), Some(3));
/// assert_eq!(num_of_integral_digits(1e5), Some(6));
/// assert_eq!(num_of_integral_digits(1e-5), Some(1));
/// assert_eq!(num_of_integral_digits(1.2345), Some(1));
/// assert_eq!(num_of_integral_digits(f64::NAN), None);
/// assert_eq!(num_of_integral_digits(f64::INFINITY), None);
/// assert_eq!(num_of_integral_digits(f64::NEG_INFINITY), None);
/// ```
#[must_use]
pub fn num_of_integral_digits(mut f: f64) -> Option<usize> {
    if f.is_nan() || f.is_infinite() {
        return None;
    }

    let mut count = 0_usize;

    if f.is_sign_negative() {
        f = -f;
        count += 1;
    }

    if f < 1.0 {
        count += 1;
    }

    #[allow(clippy::while_float)]
    while f >= 1.0 {
        f /= 10.0;
        count += 1;
    }

    Some(count)
}

/// Map a floating point number to a ansi color string.
fn float_color(f: f64) -> &'static str {
    if f.is_nan() {
        MAGENTA_TEXT
    } else if f.is_infinite() {
        YELLOW_TEXT
    } else if f.is_sign_negative() {
        RED_TEXT
    } else if f > 0.0 {
        GREEN_TEXT
    } else {
        RESET_TEXT
    }
}

// /// Pretty print a matrix.
// /// Not intended to be used directly. Use the [`pretty_print_matrix!`] macro
// /// instead.
// ///
// /// # Panics
// /// - If any of the elements of the matrix cannot be converted to a `f64`
// pub fn _pretty_print_matrix<T, M>(
//     matrix: &M,
//     name: Option<&str>,
//     file: Option<&str>,
//     line: Option<u32>,
// ) where
//     T: GbpFloat,
//     M: PrettyPrintMatrix<T>,
// {
//     let (nrows, ncols) = matrix.shape();
//     let (cell_width, _use_scientific_notation): (usize, bool) = {
//         let mut max_width = 0;
//         for i in 0..nrows {
//             for j in 0..ncols {
//                 let x = matrix.at(i, j);
//                 let width = num_of_integral_digits(x.to_f64().expect("x is
// representable as f64"))                     .unwrap_or(0)
//                     + 1;
//                 if width > max_width {
//                     max_width = width;
//                 }
//             }
//         }
//         if max_width == 0 {
//             max_width = 5; // enough for "nan" and "inf", "-inf" and "0.0"
//         }
//         let integral_digits_limit = 9;
//         let use_scientific_notation = max_width > integral_digits_limit;
//         if use_scientific_notation {
//             max_width = integral_digits_limit;
//         }
//
//         max_width += 1 + PRECISION;
//         (max_width, use_scientific_notation)
//     };
//
//     let right_padding = cell_width / 2;
//     let total_width = ncols * cell_width + right_padding;
//     let dims = format!("{nrows}x{ncols}");
//     let horizontal_line = "─".repeat(total_width);
//
//     if let (Some(file), Some(line)) = (file, line) {
//         println!("{file}:{YELLOW_TEXT}{line}{RESET_TEXT}");
//     }
//
//     // print the top border
//     if let Some(name) = name {
//         // handle if name is longer than cell_columns
//         if name.len() + dims.len() > total_width {
//
// println!("{CYAN_TEXT}{name}{RESET_TEXT}:{MAGENTA_TEXT}{dims}{RESET_TEXT}");
//
// println!("{UPPER_LEFT_CORNER}{horizontal_line}{UPPER_RIGHT_CORNER}");
//         } else {
//             println!(
//                 "{}{}{}{}{}{}{}{}{}",
//                 UPPER_LEFT_CORNER,
//                 CYAN_TEXT,
//                 name,
//                 RESET_TEXT,
//                 "─".repeat(total_width - name.len() - dims.len()),
//                 MAGENTA_TEXT,
//                 dims,
//                 RESET_TEXT,
//                 UPPER_RIGHT_CORNER
//             );
//         }
//     } else {
//         println!("{UPPER_LEFT_CORNER}{horizontal_line}{UPPER_RIGHT_CORNER}");
//     }
//
//     // print each cell in the matrix
//     for i in 0..nrows {
//         print!("{BAR}");
//         for j in 0..ncols {
//             let x = matrix.at(i, j);
//             let x = x.to_f64().expect("x is representable as f64");
//             if x.abs() > 1e6 {
//                 print!(
//                     "{}{:cell_width$.precision$e}{}",
//                     float_color(x),
//                     x,
//                     RESET_TEXT,
//                     cell_width = cell_width,
//                     precision = PRECISION
//                 );
//             } else {
//                 print!(
//                     "{}{:cell_width$.precision$}{}",
//                     float_color(x),
//                     x,
//                     RESET_TEXT,
//                     cell_width = cell_width,
//                     precision = PRECISION
//                 );
//             }
//         }
//         println!("{}{}", " ".repeat(right_padding), BAR);
//     }
//     // print the bottom border
//     println!("{LOWER_LEFT_CORNER}{horizontal_line}{LOWER_RIGHT_CORNER}");
// }

// /// Internal function to pretty print a vector
// /// Use by the [`pretty_print_vector!`] macro
// ///
// /// # Panics
// /// - If any of the elements of the matrix cannot be converted to a `f64`
// pub fn _pretty_print_vector<T, V>(
//     vector: &V,
//     name: Option<&str>,
//     file: Option<&str>,
//     line: Option<u32>,
// ) where
//     T: GbpFloat,
//     V: PrettyPrintVector<T>,
// {
//     let (cell_width, _use_scientific_notation): (usize, bool) = {
//         let mut max_width = 0;
//         for i in 0..vector.len() {
//             let x = vector.at(i);
//             let x = x.to_f64().expect("x is representable as f64");
//             let width = num_of_integral_digits(x).unwrap_or(0) + 1;
//             if width > max_width {
//                 max_width = width;
//             }
//         }
//         if max_width == 0 {
//             max_width = 4; // enough for "nan" and "inf", "-inf" and "0.0"
//         }
//         let integral_digits_limit = 9;
//         let use_scientific_notation = max_width > integral_digits_limit;
//         if use_scientific_notation {
//             max_width = integral_digits_limit;
//         }
//
//         max_width += 1 + PRECISION;
//         (max_width, use_scientific_notation)
//     };
//
//     let right_padding = cell_width / 2;
//     let total_width = vector.len() * cell_width + right_padding;
//     let horizontal_line = "─".repeat(total_width);
//     let dims = format!("{}x{}", vector.len(), 1);
//
//     if let (Some(file), Some(line)) = (file, line) {
//         println!("{}:{}{}{}", file, YELLOW_TEXT, line, RESET_TEXT);
//     }
//
//     if let Some(name) = name {
//         if name.len() + dims.len() > total_width {
//             println!(
//                 "{}{}{}:{}{}{}",
//                 CYAN_TEXT, name, RESET_TEXT, MAGENTA_TEXT, dims, RESET_TEXT
//             );
//             println!(
//                 "{}{}{}",
//                 UPPER_LEFT_CORNER, horizontal_line, UPPER_RIGHT_CORNER
//             );
//         } else {
//             println!(
//                 "{}{}{}{}{}{}{}{}{}",
//                 UPPER_LEFT_CORNER,
//                 CYAN_TEXT,
//                 name,
//                 RESET_TEXT,
//                 "─".repeat(total_width - name.len() - dims.len()),
//                 MAGENTA_TEXT,
//                 dims,
//                 RESET_TEXT,
//                 UPPER_RIGHT_CORNER
//             );
//         }
//     } else {
//         println!(
//             "{}{}{}",
//             UPPER_LEFT_CORNER, horizontal_line, UPPER_RIGHT_CORNER
//         );
//     }
//
//     print!("{}", BAR);
//
//     for i in 0..vector.len() {
//         let x = vector.at(i);
//         let x = x.to_f64().expect("x is representable as f64");
//         if x.abs() > 1e6 {
//             print!(
//                 "{}{:cell_width$.precision$e}{}",
//                 float_color(x),
//                 x,
//                 RESET_TEXT,
//                 cell_width = cell_width,
//                 precision = PRECISION
//             );
//         } else {
//             print!(
//                 "{}{:cell_width$.precision$}{}",
//                 float_color(x),
//                 x,
//                 RESET_TEXT,
//                 cell_width = cell_width,
//                 precision = PRECISION
//             );
//         }
//     }
//
//     println!("{}{}", " ".repeat(right_padding), BAR);
//
//     println!("{LOWER_LEFT_CORNER}{horizontal_line}{LOWER_RIGHT_CORNER}");
// }

/// Internal function to pretty print a vector
/// Use by the [`pretty_print_vector!`] macro
///
/// # Panics
/// - If any of the elements of the matrix cannot be converted to a `f64`
#[allow(clippy::unwrap_used, clippy::too_many_lines)]
pub fn _pretty_format_vector<T, V>(
    vector: &V,
    name: Option<&str>,
    file: Option<&str>,
    line: Option<u32>,
) -> String
where
    T: GbpFloat,
    V: PrettyPrintVector<T>,
{
    use std::fmt::Write;
    let mut text = String::new();

    let (cell_width, _use_scientific_notation): (usize, bool) = {
        let mut max_width = 0;
        for i in 0..vector.len() {
            let x = vector.at(i);
            let x = x.to_f64().expect("x is representable as f64");
            let width = num_of_integral_digits(x).unwrap_or(0) + 1;
            if width > max_width {
                max_width = width;
            }
        }
        if max_width == 0 {
            max_width = 4; // enough for "nan" and "inf", "-inf" and "0.0"
        }
        let integral_digits_limit = 9;
        let use_scientific_notation = max_width > integral_digits_limit;
        if use_scientific_notation {
            max_width = integral_digits_limit;
        }

        max_width += 1 + PRECISION;
        (max_width, use_scientific_notation)
    };

    let right_padding = cell_width / 2;
    let total_width = vector.len() * cell_width + right_padding;
    let horizontal_line = "─".repeat(total_width);
    let dims = format!("{}x{}", vector.len(), 1);

    if let (Some(file), Some(line)) = (file, line) {
        writeln!(&mut text, "{}:{}{}{}", file, YELLOW_TEXT, line, RESET_TEXT).unwrap();
    }

    if let Some(name) = name {
        if name.len() + dims.len() > total_width {
            writeln!(
                &mut text,
                "{}{}{}:{}{}{}",
                CYAN_TEXT, name, RESET_TEXT, MAGENTA_TEXT, dims, RESET_TEXT
            )
            .unwrap();

            writeln!(
                &mut text,
                "{}{}{}",
                UPPER_LEFT_CORNER, horizontal_line, UPPER_RIGHT_CORNER
            )
            .unwrap();
        } else {
            writeln!(
                &mut text,
                "{}{}{}{}{}{}{}{}{}",
                UPPER_LEFT_CORNER,
                CYAN_TEXT,
                name,
                RESET_TEXT,
                "─".repeat(total_width - name.len() - dims.len()),
                MAGENTA_TEXT,
                dims,
                RESET_TEXT,
                UPPER_RIGHT_CORNER
            )
            .unwrap();
        }
    } else {
        writeln!(
            &mut text,
            "{}{}{}",
            UPPER_LEFT_CORNER, horizontal_line, UPPER_RIGHT_CORNER
        )
        .unwrap();
    }

    write!(&mut text, "{}", BAR).unwrap();

    for i in 0..vector.len() {
        let x = vector.at(i);
        let x = x.to_f64().expect("x is representable as f64");
        if x.abs() > 1e6 {
            write!(
                &mut text,
                "{}{:cell_width$.precision$e}{}",
                float_color(x),
                x,
                RESET_TEXT,
                cell_width = cell_width,
                precision = PRECISION
            )
            .unwrap();
        } else {
            write!(
                &mut text,
                "{}{:cell_width$.precision$}{}",
                float_color(x),
                x,
                RESET_TEXT,
                cell_width = cell_width,
                precision = PRECISION
            )
            .unwrap();
        }
    }

    writeln!(&mut text, "{}{}", " ".repeat(right_padding), BAR).unwrap();

    writeln!(
        &mut text,
        "{LOWER_LEFT_CORNER}{horizontal_line}{LOWER_RIGHT_CORNER}"
    )
    .unwrap();

    text
}

/// Extension trait that adds a [`pretty_print`] method to vectors
#[allow(clippy::len_without_is_empty)]
pub trait PrettyPrintVector<T: GbpFloat>: Sized {
    /// Returns the length of the vector.
    #[allow(clippy::len_without_is_empty)]
    fn len(&self) -> usize;
    /// Returns the element at index `i`.
    fn at(&self, i: usize) -> T;

    /// Pretty prints the vector.
    #[inline(always)]
    fn pretty_print(&self) {
        println!("{}", _pretty_format_vector(self, None, None, None));
        // _pretty_print_vector(self, None, None, None);
    }

    /// Pretty format the vector.
    fn pretty_format(&self) -> String {
        _pretty_format_vector(self, None, None, None)
    }
}

impl<T: GbpFloat> PrettyPrintVector<T> for Vector<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn at(&self, i: usize) -> T {
        self[i]
    }
}
/// Extension trait that adds a [`pretty_print`] method to matrices
pub trait PrettyPrintMatrix<T: GbpFloat>: Sized {
    /// Returns the shape of the matrix as a tuple `(nrows, ncols)`.
    fn shape(&self) -> (usize, usize);
    /// Returns the element at index `(i, j)`.
    fn at(&self, i: usize, j: usize) -> T;

    /// Pretty prints the matrix.
    #[inline(always)]
    fn pretty_print(&self) {
        println!("{}", _pretty_format_matrix(self, None, None, None));
        // _pretty_print_matrix(self, None, None, None);
    }

    /// Pretty format the matrix.
    fn pretty_format(&self) -> String {
        _pretty_format_matrix(self, None, None, None)
    }
}

impl<T: GbpFloat> PrettyPrintMatrix<T> for Matrix<T> {
    #[inline(always)]
    fn shape(&self) -> (usize, usize) {
        (self.nrows(), self.ncols())
    }

    #[inline(always)]
    fn at(&self, i: usize, j: usize) -> T {
        self[(i, j)]
    }
}

#[allow(
    clippy::unwrap_used,
    clippy::missing_panics_doc,
    clippy::too_many_lines
)]
pub fn _pretty_format_matrix<T, M>(
    matrix: &M,
    name: Option<&str>,
    file: Option<&str>,
    line: Option<u32>,
) -> String
where
    T: GbpFloat,
    M: PrettyPrintMatrix<T>,
{
    use std::fmt::Write;

    let (nrows, ncols) = matrix.shape();
    let (cell_width, _use_scientific_notation): (usize, bool) = {
        let mut max_width = 0;
        for i in 0..nrows {
            for j in 0..ncols {
                let x = matrix.at(i, j);
                let width = num_of_integral_digits(x.to_f64().expect("x is representable as f64"))
                    .unwrap_or(0)
                    + 1;
                if width > max_width {
                    max_width = width;
                }
            }
        }
        if max_width == 0 {
            max_width = 5; // enough for "nan" and "inf", "-inf" and "0.0"
        }
        let integral_digits_limit = 9;
        let use_scientific_notation = max_width > integral_digits_limit;
        if use_scientific_notation {
            max_width = integral_digits_limit;
        }

        max_width += 1 + PRECISION;
        (max_width, use_scientific_notation)
    };

    let right_padding = cell_width / 2;
    let total_width = ncols * cell_width + right_padding;
    let dims = format!("{nrows}x{ncols}");
    let horizontal_line = "─".repeat(total_width);
    let mut text = String::new();

    if let (Some(file), Some(line)) = (file, line) {
        writeln!(&mut text, "{file}:{YELLOW_TEXT}{line}{RESET_TEXT}").unwrap();
    }

    // print the top border
    if let Some(name) = name {
        // handle if name is longer than cell_columns
        if name.len() + dims.len() > total_width {
            writeln!(
                &mut text,
                "{CYAN_TEXT}{name}{RESET_TEXT}:{MAGENTA_TEXT}{dims}{RESET_TEXT}"
            )
            .unwrap();
            writeln!(
                &mut text,
                "{UPPER_LEFT_CORNER}{horizontal_line}{UPPER_RIGHT_CORNER}"
            )
            .unwrap();
        } else {
            writeln!(
                &mut text,
                "{}{}{}{}{}{}{}{}{}",
                UPPER_LEFT_CORNER,
                CYAN_TEXT,
                name,
                RESET_TEXT,
                "─".repeat(total_width - name.len() - dims.len()),
                MAGENTA_TEXT,
                dims,
                RESET_TEXT,
                UPPER_RIGHT_CORNER
            )
            .unwrap();
        }
    } else {
        writeln!(
            &mut text,
            "{UPPER_LEFT_CORNER}{horizontal_line}{UPPER_RIGHT_CORNER}"
        )
        .unwrap();
    }

    // print each cell in the matrix
    for i in 0..nrows {
        write!(&mut text, "{BAR}").unwrap();
        for j in 0..ncols {
            let x = matrix.at(i, j);
            let x = x.to_f64().expect("x is representable as f64");
            if x.abs() > 1e6 {
                write!(
                    &mut text,
                    "{}{:cell_width$.precision$e}{}",
                    float_color(x),
                    x,
                    RESET_TEXT,
                    cell_width = cell_width,
                    precision = PRECISION
                )
                .unwrap();
            } else {
                write!(
                    &mut text,
                    "{}{:cell_width$.precision$}{}",
                    float_color(x),
                    x,
                    RESET_TEXT,
                    cell_width = cell_width,
                    precision = PRECISION
                )
                .unwrap();
            }
        }
        writeln!(&mut text, "{}{}", " ".repeat(right_padding), BAR).unwrap();
    }
    // print the bottom border
    writeln!(
        &mut text,
        "{LOWER_LEFT_CORNER}{horizontal_line}{LOWER_RIGHT_CORNER}"
    )
    .unwrap();

    text
}

/// Pretty prints a vector
#[macro_export]
macro_rules! pretty_print_vector {
    ($name:expr) => {
        println!(
            "{}",
            $crate::pretty_print::_pretty_format_vector(
                $name,
                Some(stringify!($name)),
                Some(file!()),
                Some(line!()),
            )
        );
    };
    ($name:expr, None) => {
        println!(
            "{}",
            $crate::pretty_print::_pretty_format_vector($name, None, None, None)
        );
    };
    ($name:literal, $vector:expr) => {
        println!(
            "{}",
            $crate::pretty_print::_pretty_format_vector(
                $vector,
                Some($name),
                Some(file!()),
                Some(line!()),
            )
        );
    };
}

/// Pretty prints a matrix
#[macro_export]
macro_rules! pretty_print_matrix {
    ($name:expr) => {
        println!(
            "{}",
            $crate::pretty_print::_pretty_format_matrix(
                $name,
                Some(stringify!($name)),
                Some(file!()),
                Some(line!()),
            )
        );
    };
    ($name:expr, None) => {
        println!(
            "{}",
            $crate::pretty_print::_pretty_format_matrix($name, None, None, None)
        );
    };

    ($name:literal, $matrix:expr) => {
        println!(
            "{}",
            $crate::pretty_print::_pretty_format_matrix(
                $matrix,
                Some($name),
                Some(file!()),
                Some(line!()),
            )
        );
    };
}

#[macro_export]
macro_rules! pretty_format_matrix {
    ($name:expr) => {
        $crate::pretty_print::_pretty_format_matrix(
            $name,
            Some(stringify!($name)),
            Some(file!()),
            Some(line!()),
        );
    };

    ($name:expr, None) => {
        $crate::pretty_print::_pretty_format_matrix($name, Some(stringify!($name)), None, None);
    };

    ($name:literal, $matrix:expr, None) => {
        $crate::pretty_print::_pretty_format_matrix($matrix, Some($name), None, None);
    };

    ($name:literal, $matrix:expr) => {
        $crate::pretty_print::_pretty_format_matrix(
            $matrix,
            Some($name),
            Some(file!()),
            Some(line!()),
        );
    };
}

#[macro_export]
macro_rules! pretty_format_vector {
    ($name:expr) => {
        $crate::pretty_print::_pretty_format_vector(
            $name,
            Some(stringify!($name)),
            Some(file!()),
            Some(line!()),
        );
    };

    ($name:expr, None) => {
        $crate::pretty_print::_pretty_format_vector($name, None, None, None);
    };

    ($name:literal, $vector:expr, None) => {
        $crate::pretty_print::_pretty_format_vector($vector, Some($name), None, None);
    };

    ($name:literal, $vector:expr) => {
        $crate::pretty_print::_pretty_format_vector(
            $vector,
            Some($name),
            Some(file!()),
            Some(line!()),
        );
    };
}
