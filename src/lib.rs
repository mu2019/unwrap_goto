
#[macro_export]
macro_rules! some_or_continue{
    ($e:expr) => {
        {
            if $e.is_none() {
                continue;
            } else {
                $e.unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_break{
    ($e:expr) => {
        {
            if $e.is_none() {
                break;
            } else {
                $e.unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_return {
    ($e:expr) => {
        {
            if $e.is_none() {
                return;
            } else {
                $e.unwrap()
            }
        }
    };
    ($e:expr, $r: expr) => {
        {
            if $e.is_none() {
                return $r;
            } else {
                $e.unwrap()
            }
        }
    };    
}

#[macro_export]
macro_rules! ok_or_continue{
    ($e:expr) => {
        {
            if $e.is_err() {
                continue;
            } else {
                $e.unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_return{
    ($e:expr) => {
        {
            if $e.is_err() {
                return;
            } else {
                $e.unwrap()
            }
        }
    };
    ($e:expr, $r: expr) => {
        {
            if $e.is_err() {
                return $r;
            } else {
                $e.unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_break{
    ($e:expr) => {
        {
            if $e.is_err() {
                break;
            } else {
                $e.unwrap()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn none_is_continue() {
        let m = vec![Some(1), None, Some(2)];
        let mut k = 0;
        for j in m {
            println!("income value:{:?}", j);
            k = some_or_continue!(j);
            println!("wrap value:{:?}", k);            
        }
        println!("final value:{:?}", k);
        assert_eq!(k, 2);
    }

    #[test]
    fn none_is_break() {
        let m = vec![Some(1), None, Some(2)];
        let mut k = 0;
        for j in m {
            println!("income value:{:?}", j);
            k = some_or_break!(j);
            println!("wrap value:{:?}", k);            
        }
        println!("final value:{:?}", k);
        assert_eq!(k, 1);
    }

    fn do_none_is_return(m: &Vec<Option<i32>>) -> i64 {
        let mut k = 0;
        for v in m {
            println!("imcome:{:?}", v);
            k = some_or_return!(v, 8);
            println!("wrap value:{:?}", k);
        }
        assert_eq!(k, 1);
        0
    }

    #[test]
    fn none_is_return() {
        let m = vec![Some(1), None, Some(1)];
        let n = do_none_is_return(&m);
        println!("return value:{:?}", n);
    }    
    
}
