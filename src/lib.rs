
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
    ($e:expr, $log_info: expr) => {
        {
            if $e.is_none() {
                $log_info;
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
    ($e:expr, $return_value: expr, $log_info: expr) => {
        {
            if $e.is_none() {
                $log_info;
                return $return_value;
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
    ($e:expr, $log_info: expr) => {
        {
            if $e.is_err() {
                $log_info;
                continue;
            } else {
                $e.unwrap()
            }
        }
    };    
}


/// Unwrap arguments when it represents ([`Ok`]).
///
/// otherwise will return ([`()`]) or second arguments.
/// # Examples
///
/// ```edition2018
///
/// #[macro_use]
/// extern crate unwrap_goto;
/// 
/// fn when_err_and_return() {
///     let x: Result<i32, &str> = Err("Some error message");
///     ok_or_return!(x);
///     println!("anything print out:{:?}", x);
/// }
/// ``` 
/// 
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
    ($e:expr, $return_value: expr, $log_info: expr) => {
        {
            if $e.is_err() {
                $log_info;
                return $return_value;
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
    ($e:expr, $log_info: expr) => {
        {
            if $e.is_err() {
                $log_info;
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
    

    #[test]
    fn when_err_and_return() {
        let x: Result<i32, &str> = Err("Some error message");
        ok_or_return!(x);
        println!("anything print out:{:?}", x);
    }

    
    fn when_err_and_return_something() -> i32 {
        let x: Result<i32, &str> = Err("Some error message");
        ok_or_return!(x, -1);
        println!("anything print out:{:?}", x);
        0
    }

    #[test]
    fn call_when_err_and_return_something() {
        let x = when_err_and_return_something();
        println!("call when_err_and_return_something return: {:?}", x);
        assert!(x == -1);
    }


    #[test]
    fn call_when_err_return_log() {
        use log::error;

        let x: Result<i32, &str> = Err("Some error message");
        ok_or_return!(x, (), error!("log error info.{:?}", x));
        println!("anything print out");
    }




}
