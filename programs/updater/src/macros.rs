#[macro_export]
macro_rules! validate {
    ($assert:expr, $err:expr) => {
        {
            if ($assert) {
                Ok(())
            } else {
                let error_code: ErrorCode = $err;
                msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
                Err(error_code)
            }
        }
    };
    (
        $assert:expr,
        $err:expr,
        $($arg:tt)+
    ) => {
        {
        if ($assert) {
            Ok(())
        } else {
            let error_code: ErrorCode = $err;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            msg!($($arg)*);
            Err(error_code)
        }
        }
    };
}

#[macro_export]
macro_rules! load_mut {
    ($account_loader:expr) => {
        {
        $account_loader.load_mut().map_err(|e| {
            msg!("e {:?}", e);
            let error_code = ErrorCode::UnableToLoadAccountLoader;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            error_code
        })
        }
    };
}

#[macro_export]
macro_rules! load {
    ($account_loader:expr) => {
        {
        $account_loader.load().map_err(|_| {
            let error_code = ErrorCode::UnableToLoadAccountLoader;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            error_code
        })
        }
    };
}

#[macro_export]
macro_rules! safe_increment {
    ($struct:expr, $value:expr) => {
        {
        $struct = $struct.checked_add($value).ok_or_else(math_error!())?
        }
    };
}

#[macro_export]
macro_rules! safe_decrement {
    ($struct:expr, $value:expr) => {
        {
        $struct = $struct.checked_sub($value).ok_or_else(math_error!())?
        }
    };
}
