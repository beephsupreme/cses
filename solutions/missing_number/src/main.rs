/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

 use anyhow::Error;

 use missing_number::missing_number;
 use utils::io::{get_token, get_vector, load_all_tokens};

 /// Driver for the missing number problem from CSES problem set (https://cses.fi/problemset/task/1083).
 /// Given a sequence of n - 1 distinct integers in the range 1, 2, ..., n, find the missing number.
 fn main() -> Result<(), Error> {
     let reader = std::io::BufReader::new(std::io::stdin());
     let mut buffer: String = String::new();
     let mut tokens = load_all_tokens(reader, &mut buffer)?;
     let n: u64 = get_token(&mut tokens)?;
     let v: Vec<u64> = get_vector(&mut tokens)?;
     let r: u64 = missing_number(n, v)?;
     println!("{}", r);
     Ok(())
 }
 