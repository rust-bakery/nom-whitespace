#[doc(hidden)]
#[macro_export]
macro_rules! nom_ws_error_position (
  ($($args:tt)*) => (error_position!($($args)*));
);

#[doc(hidden)]
#[macro_export]
macro_rules! nom_ws_error_node_position (
  ($($args:tt)*) => (error_node_position!($($args)*));
);

#[doc(hidden)]
#[macro_export]
macro_rules! nom_call (
  ($($args:tt)*) => (call!($($args)*));
);

#[doc(hidden)]
#[macro_export]
macro_rules! nom_tuple (
  ($($args:tt)*) => (tuple!($($args)*));
);

#[doc(hidden)]
#[macro_export]
macro_rules! nom_permutation_init (
  ($($args:tt)*) => (permutation_init!($($args)*));
);

#[doc(hidden)]
#[macro_export]
macro_rules! nom_permutation_unwrap (
  ($($args:tt)*) => (permutation_unwrap!($($args)*));
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! succ (
  (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
  (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
  (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
  (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
  (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
  (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
  (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
  (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
  (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
  (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
  (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
  (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
  (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
  (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
  (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
  (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
  (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
  (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
  (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
  (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
);

// HACK: for some reason, Rust 1.11 does not accept $res.$it in
// nom_permutation_unwrap. This is a bit ugly, but it will have no
// impact on the generated code
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! acc (
  (0, $tup:expr) => ($tup.0);
  (1, $tup:expr) => ($tup.1);
  (2, $tup:expr) => ($tup.2);
  (3, $tup:expr) => ($tup.3);
  (4, $tup:expr) => ($tup.4);
  (5, $tup:expr) => ($tup.5);
  (6, $tup:expr) => ($tup.6);
  (7, $tup:expr) => ($tup.7);
  (8, $tup:expr) => ($tup.8);
  (9, $tup:expr) => ($tup.9);
  (10, $tup:expr) => ($tup.10);
  (11, $tup:expr) => ($tup.11);
  (12, $tup:expr) => ($tup.12);
  (13, $tup:expr) => ($tup.13);
  (14, $tup:expr) => ($tup.14);
  (15, $tup:expr) => ($tup.15);
  (16, $tup:expr) => ($tup.16);
  (17, $tup:expr) => ($tup.17);
  (18, $tup:expr) => ($tup.18);
  (19, $tup:expr) => ($tup.19);
  (20, $tup:expr) => ($tup.20);
);

#[macro_export(local_inner_macros)]
macro_rules! wrap_sep (
  ($i:expr, $separator:expr, $submac:ident!( $($args:tt)* )) => ({
    use $crate::lib::std::result::Result::*;
    use $crate::lib::nom::{Err,Convert,IResult};

    fn unify_types<I,O,P,E>(_: &IResult<I,O,E>, _: &IResult<I,P,E>) {}

    let sep_res = ($separator)($i);
    match sep_res {
      Ok((i1,_))    => {
        let res = $submac!(i1, $($args)*);
        unify_types(&sep_res, &res);
        res
      },
      Err(e) => Err(Err::convert(e)),
    }
  });
  ($i:expr, $separator:expr, $f:expr) => (
    wrap_sep!($i, $separator, nom_nom_call!($f))
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! pair_sep (
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    nom_tuple!(
      $i,
      sep!($separator, $submac!($($args)*)),
      sep!($separator, $submac2!($($args2)*))
    )
  );
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $g:expr) => (
    pair_sep!($i, $separator, $submac!($($args)*), nom_call!($g));
  );
  ($i:expr, $separator:path, $f:expr, $submac:ident!( $($args:tt)* )) => (
    pair_sep!($i, $separator, nom_call!($f), $submac!($($args)*));
  );
  ($i:expr, $separator:path, $f:expr, $g:expr) => (
    pair_sep!($i, $separator, nom_call!($f), nom_call!($g));
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! delimited_sep (
  ($i:expr, $separator:path, $submac1:ident!( $($args1:tt)* ), $($rest:tt)+) => ({
    use $crate::lib::std::result::Result::*;

    match tuple_sep!($i, $separator, (), $submac1!($($args1)*), $($rest)*) {
      Err(e) => Err(e),
      Ok((remaining, (_,o,_))) => {
        Ok((remaining, o))
      }
    }
  });
  ($i:expr, $separator:path, $f:expr, $($rest:tt)+) => (
    delimited_sep!($i, $separator, nom_call!($f), $($rest)*);
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! separated_pair_sep (
  ($i:expr, $separator:path, $submac1:ident!( $($args1:tt)* ), $($rest:tt)+) => ({
    use $crate::lib::std::result::Result::*;

    match tuple_sep!($i, $separator, (), $submac1!($($args1)*), $($rest)*) {
      Err(e) => Err(e),
      Ok((remaining, (o1,_,o2))) => {
        Ok((remaining, (o1,o2)))
      }
    }
  });
  ($i:expr, $separator:path, $f:expr, $($rest:tt)+) => (
    separated_pair_sep!($i, $separator, nom_call!($f), $($rest)*);
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! preceded_sep (
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => ({
    use $crate::lib::std::result::Result::*;

    match pair_sep!($i, $separator, $submac!($($args)*), $submac2!($($args2)*)) {
      Err(e) => Err(e),
      Ok((remaining, (_,o))) => {
        Ok((remaining, o))
      }
    }
  });
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $g:expr) => (
    preceded_sep!($i, $separator, $submac!($($args)*), nom_call!($g));
  );
  ($i:expr, $separator:path, $f:expr, $submac:ident!( $($args:tt)* )) => (
    preceded_sep!($i, $separator, nom_call!($f), $submac!($($args)*));
  );
  ($i:expr, $separator:path, $f:expr, $g:expr) => (
    preceded_sep!($i, $separator, nom_call!($f), nom_call!($g));
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! terminated_sep (
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => ({
    use $crate::lib::std::result::Result::*;

    match pair_sep!($i, $separator, $submac!($($args)*), $submac2!($($args2)*)) {
      Err(e) => Err(e),
      Ok((remaining, (o,_))) => {
        Ok((remaining, o))
      }
    }
  });
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $g:expr) => (
    terminated_sep!($i, $separator, $submac!($($args)*), nom_call!($g));
  );
  ($i:expr, $separator:path, $f:expr, $submac:ident!( $($args:tt)* )) => (
    terminated_sep!($i, $separator, nom_call!($f), $submac!($($args)*));
  );
  ($i:expr, $separator:path, $f:expr, $g:expr) => (
    terminated_sep!($i, $separator, nom_call!($f), nom_call!($g));
  );
);

/// Internal parser, do not use directly
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! tuple_sep (
  ($i:expr, $separator:path, ($($parsed:tt),*), $e:path, $($rest:tt)*) => (
    tuple_sep!($i, $separator, ($($parsed),*), nom_call!($e), $($rest)*);
  );
  ($i:expr, $separator:path, (), $submac:ident!( $($args:tt)* ), $($rest:tt)*) => (
    {
      use $crate::lib::std::result::Result::*;

      match sep!($i, $separator, $submac!($($args)*)) {
        Err(e) => Err(e),
        Ok((i,o))     => {
          tuple_sep!(i, $separator, (o), $($rest)*)
        }
      }
    }
  );
  ($i:expr, $separator:path, ($($parsed:tt)*), $submac:ident!( $($args:tt)* ), $($rest:tt)*) => (
    {
      use $crate::lib::std::result::Result::*;

      match sep!($i, $separator, $submac!($($args)*)) {
        Err(e) => Err(e),
        Ok((i,o))     => {
          tuple_sep!(i, $separator, ($($parsed)* , o), $($rest)*)
        }
      }
    }
  );
  ($i:expr, $separator:path, ($($parsed:tt),*), $e:path) => (
    tuple_sep!($i, $separator, ($($parsed),*), nom_call!($e));
  );
  ($i:expr, $separator:path, (), $submac:ident!( $($args:tt)* )) => (
    {
      use $crate::lib::std::result::Result::*;

      match sep!($i, $separator, $submac!($($args)*)) {
        Err(e) => Err(e),
        Ok((i,o))     => {
          Ok((i, (o)))
        }
      }
    }
  );
  ($i:expr, $separator:path, ($($parsed:expr),*), $submac:ident!( $($args:tt)* )) => (
    {
      use $crate::lib::std::result::Result::*;

      match sep!($i, $separator, $submac!($($args)*)) {
        Err(e) => Err(e),
        Ok((i,o))     => {
          Ok((i, ($($parsed),* , o)))
        }
      }
    }
  );
  ($i:expr, $separator:path, ($($parsed:expr),*)) => (
    {
      ::sts::result::Result::Ok(($i, ($($parsed),*)))
    }
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! do_parse_sep (
  (__impl $i:expr, $separator:path, ( $($rest:expr),* )) => (
    $crate::lib::std::result::Result::Ok(($i, ( $($rest),* )))
  );

  (__impl $i:expr, $separator:path, $e:ident >> $($rest:tt)*) => (
    do_parse_sep!(__impl $i, $separator, nom_call!($e) >> $($rest)*);
  );
  (__impl $i:expr, $separator:path, $submac:ident!( $($args:tt)* ) >> $($rest:tt)*) => (
    {
      use $crate::lib::std::result::Result::*;

      match sep!($i, $separator, $submac!($($args)*)) {
        Err(e) => Err(e),
        Ok((i,_))     => {
          do_parse_sep!(__impl i, $separator, $($rest)*)
        },
      }
    }
  );

  (__impl $i:expr, $separator:path, $field:ident : $e:ident >> $($rest:tt)*) => (
    do_parse_sep!(__impl $i, $separator, $field: nom_call!($e) >> $($rest)*);
  );

  (__impl $i:expr, $separator:path, $field:ident : $submac:ident!( $($args:tt)* ) >> $($rest:tt)*) => (
    {
      use $crate::lib::std::result::Result::*;

      match sep!($i, $separator, $submac!($($args)*)) {
        Err(e) => Err(e),
        Ok((i,o))     => {
          let $field = o;
          do_parse_sep!(__impl i, $separator, $($rest)*)
        },
      }
    }
  );

  // ending the chain
  (__impl $i:expr, $separator:path, $e:ident >> ( $($rest:tt)* )) => (
    do_parse_sep!(__impl $i, $separator, nom_call!($e) >> ( $($rest)* ));
  );

  (__impl $i:expr, $separator:path, $submac:ident!( $($args:tt)* ) >> ( $($rest:tt)* )) => ({
    use $crate::lib::std::result::Result::*;

    match sep!($i, $separator, $submac!($($args)*)) {
      Err(e) => Err(e),
      Ok((i,_))     => {
        Ok((i, ( $($rest)* )))
      },
    }
  });

  (__impl $i:expr, $separator:path, $field:ident : $e:ident >> ( $($rest:tt)* )) => (
    do_parse_sep!(__impl $i, $separator, $field: nom_call!($e) >> ( $($rest)* ) );
  );

  (__impl $i:expr, $separator:path, $field:ident : $submac:ident!( $($args:tt)* ) >> ( $($rest:tt)* )) => ({
    use $crate::lib::std::result::Result::*;

    match sep!($i, $separator, $submac!($($args)*)) {
      Err(e) => Err(e),
      Ok((i,o))     => {
        let $field = o;
        Ok((i, ( $($rest)* )))
      },
    }
  });

  ($i:expr, $separator:path, $($rest:tt)*) => (
    {
      do_parse_sep!(__impl $i, $separator, $($rest)*)
    }
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! permutation_sep (
  ($i:expr, $separator:path, $($rest:tt)*) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::lib::std::option::Option::*;
      use $crate::lib::nom::{Err,ErrorKind,Convert};

      let mut res    = nom_permutation_init!((), $($rest)*);
      let mut input  = $i;
      let mut error  = None;
      let mut needed = None;

      loop {
        let mut all_done = true;
        permutation_iterator_sep!(0, input, $separator, all_done, needed, res, $($rest)*);

        //if we reach that part, it means none of the parsers were able to read anything
        if !all_done {
          //FIXME: should wrap the error returned by the child parser
          error = Option::Some(nom_ws_error_position!(input, ErrorKind::Permutation));
        }
        break;
      }

      if let Some(need) = needed {
        Err(Err::convert(need))
      } else {
        if let Some(unwrapped_res) = { nom_permutation_unwrap!(0, (), res, $($rest)*) } {
          Ok((input, unwrapped_res))
        } else {
          if let Some(e) = error {
            Err(Err::Error(nom_ws_error_node_position!($i, ErrorKind::Permutation, e)))
          } else {
            Err(Err::Error(nom_ws_error_position!($i, ErrorKind::Permutation)))
          }
        }
      }
    }
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! permutation_iterator_sep (
  ($it:tt,$i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $e:ident?, $($rest:tt)*) => (
    permutation_iterator_sep!($it, $i, $separator, $all_done, $needed, $res, nom_call!($e), $($rest)*);
  );
  ($it:tt,$i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $e:ident, $($rest:tt)*) => (
    permutation_iterator_sep!($it, $i, $separator, $all_done, $needed, $res, nom_call!($e), $($rest)*);
  );

  ($it:tt, $i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $submac:ident!( $($args:tt)* )?, $($rest:tt)*) => ({
    permutation_iterator_sep!($it, $i, $separator, $all_done, $needed, $res, $submac!($($args)*), $($rest)*);
  });
  ($it:tt, $i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $submac:ident!( $($args:tt)* ), $($rest:tt)*) => ({
    use $crate::lib::std::result::Result::*;
    use $crate::lib::nom::Err;

    if acc!($it, $res) == $crate::lib::std::option::Option::None {
      match {sep!($i, $separator, $submac!($($args)*))} {
        Ok((i,o))     => {
          $i = i;
          acc!($it, $res) = $crate::lib::std::option::Option::Some(o);
          continue;
        },
        Err(Err::Error(_)) => {
          $all_done = false;
        },
        Err(e) => {
          $needed = $crate::lib::std::option::Option::Some(e);
          break;
        }
      };
    }
    succ!($it, permutation_iterator_sep!($i, $separator, $all_done, $needed, $res, $($rest)*));
  });

  ($it:tt,$i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $e:ident?) => (
    permutation_iterator_sep!($it, $i, $separator, $all_done, $res, nom_call!($e));
  );
  ($it:tt,$i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $e:ident) => (
    permutation_iterator_sep!($it, $i, $separator, $all_done, $res, nom_call!($e));
  );

  ($it:tt, $i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $submac:ident!( $($args:tt)* )?) => ({
    permutation_iterator_sep!($it, $i, $separator, $all_done, $needed, $res, $submac!($($args)*));
  });
  ($it:tt, $i:expr, $separator:path, $all_done:expr, $needed:expr, $res:expr, $submac:ident!( $($args:tt)* )) => ({
    use $crate::lib::std::result::Result::*;
    use $crate::lib::nom::Err;

    if acc!($it, $res) == $crate::lib::std::option::Option::None {
      match sep!($i, $separator, $submac!($($args)*)) {
        Ok((i,o))     => {
          $i = i;
          acc!($it, $res) = $crate::lib::std::option::Option::Some(o);
          continue;
        },
        Err(Err::Error(_)) => {
          $all_done = false;
        },
        Err(e) => {
          $needed = $crate::lib::std::option::Option::Some(e);
          break;
        }
      };
    }
  });
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! alt_sep (
  (__impl $i:expr, $separator:path, $e:path | $($rest:tt)*) => (
    alt_sep!(__impl $i, $separator, nom_call!($e) | $($rest)*);
  );

  (__impl $i:expr, $separator:path, $subrule:ident!( $($args:tt)*) | $($rest:tt)*) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::lib::nom::Err;

      let res = sep!($i, $separator, $subrule!($($args)*));
      match res {
        Ok((_,_))          => res,
        Err(Err::Error(_)) => alt_sep!(__impl $i, $separator, $($rest)*),
        Err(e)            => Err(e),
      }
    }
  );

  (__impl $i:expr, $separator:path, $subrule:ident!( $($args:tt)* ) => { $gen:expr } | $($rest:tt)+) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::lib::nom::Err;

      match sep!($i, $separator, $subrule!( $($args)* )) {
        Ok((i,o))               => Ok((i,$gen(o))),
        Err(Err::Error(_))      => {
          alt_sep!(__impl $i, $separator, $($rest)*)
        },
        Err(e)            => Err(e),
      }
    }
  );

  (__impl $i:expr, $separator:path, $e:path => { $gen:expr } | $($rest:tt)*) => (
    alt_sep!(__impl $i, $separator, nom_call!($e) => { $gen } | $($rest)*);
  );

  (__impl $i:expr, $separator:path, $e:path => { $gen:expr }) => (
    alt_sep!(__impl $i, $separator, nom_call!($e) => { $gen });
  );

  (__impl $i:expr, $separator:path, $subrule:ident!( $($args:tt)* ) => { $gen:expr }) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::lib::nom::Err;

      match sep!($i, $separator, $subrule!( $($args)* )) {
        Ok((i,o))     => Ok((i,$gen(o))),
        Err(Err::Error(e))      => {
          fn unify_types<T>(_: &T, _: &T) {}
          let e2 = nom_ws_error_position!($i, $crate::lib::nom::ErrorKind::Alt);
          unify_types(&e, &e2);
          Err(Err::Error(e2))
        },
        Err(e)            => Err(e),
      }
    }
  );

  (__impl $i:expr, $separator:path, $e:path) => (
    alt_sep!(__impl $i, $separator, nom_call!($e));
  );

  (__impl $i:expr, $separator:path, $subrule:ident!( $($args:tt)*)) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::lib::nom::Err;

      match sep!($i, $separator, $subrule!( $($args)* )) {
        Ok((i,o))     => Ok((i,o)),
        Err(Err::Error(e))      => {
          fn unify_types<T>(_: &T, _: &T) {}
          let e2 = nom_ws_error_position!($i, $crate::lib::nom::ErrorKind::Alt);
          unify_types(&e, &e2);
          Err(Err::Error(e2))
        },
        Err(e)            => Err(e),
      }
    }
  );

  (__impl $i:expr) => ({
    use $crate::lib::std::result::Result::*;
    use $crate::lib::nom::{Err,Needed,IResult};

    Err(Err::Error(nom_ws_error_position!($i, $crate::lib::nom::ErrorKind::Alt)))
  });

  (__impl $i:expr, $separator:path) => ({
    use $crate::lib::std::result::Result::*;
    use $crate::lib::nom::{Err,Needed,IResult};

    Err(Err::Error(nom_ws_error_position!($i, $crate::lib::nom::ErrorKind::Alt)))
  });

  ($i:expr, $separator:path, $($rest:tt)*) => (
    {
      alt_sep!(__impl $i, $separator, $($rest)*)
    }
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! alt_complete_sep (
  ($i:expr, $separator:path, $e:path | $($rest:tt)*) => (
    alt_complete_sep!($i, $separator, complete!(nom_call!($e)) | $($rest)*);
  );

  ($i:expr, $separator:path, $subrule:ident!( $($args:tt)*) | $($rest:tt)*) => (
    {
      use $crate::lib::std::result::Result::*;

      let res = complete!($i, sep!($separator, $subrule!($($args)*)));
      match res {
        Ok((_,_)) => res,
        _ => alt_complete_sep!($i, $separator, $($rest)*),
      }
    }
  );

  ($i:expr, $separator:path, $subrule:ident!( $($args:tt)* ) => { $gen:expr } | $($rest:tt)+) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::lib::nom::{Err,Needed,IResult};

      match complete!($i, sep!($separator, $subrule!($($args)*))) {
        Ok((i,o)) => Ok((i,$gen(o))),
        _ => alt_complete_sep!($i, $separator, $($rest)*),
      }
    }
  );

  ($i:expr, $separator:path, $e:path => { $gen:expr } | $($rest:tt)*) => (
    alt_complete_sep!($i, $separator, complete!(nom_call!($e)) => { $gen } | $($rest)*);
  );

  // Tail (non-recursive) rules

  ($i:expr, $separator:path, $e:path => { $gen:expr }) => (
    alt_complete_sep!($i, $separator, nom_call!($e) => { $gen });
  );

  ($i:expr, $separator:path, $subrule:ident!( $($args:tt)* ) => { $gen:expr }) => (
    alt_sep!(__impl $i, $separator, complete!($subrule!($($args)*)) => { $gen })
  );

  ($i:expr, $separator:path, $e:path) => (
    alt_complete_sep!($i, $separator, nom_call!($e));
  );

  ($i:expr, $separator:path, $subrule:ident!( $($args:tt)*)) => (
    alt_sep!(__impl $i, $separator, complete!($subrule!($($args)*)))
  );
);

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! switch_sep (
  (__impl $i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $($p:pat => $subrule:ident!( $($args2:tt)* ))|* ) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::lib::nom::Err;

      match sep!($i, $separator, $submac!($($args)*)) {
        Err(Err::Error(e))      => Err(Err::Error(nom_ws_error_node_position!(
            $i, $crate::lib::nom::ErrorKind::Switch, e
        ))),
        Err(Err::Failure(e))    => Err(Err::Failure(
            nom_ws_error_node_position!($i, $crate::lib::nom::ErrorKind::Switch, e))),
        Err(e) => Err(e),
        Ok((i, o))    => {
          match o {
            $($p => match sep!(i, $separator, $subrule!($($args2)*)) {
              Err(Err::Error(e)) => Err(Err::Error(nom_ws_error_node_position!(
                  $i, $crate::lib::nom::ErrorKind::Switch, e
              ))),
              Err(Err::Failure(e))    => Err(Err::Failure(
                  nom_ws_error_node_position!($i, $crate::lib::nom::ErrorKind::Switch, e))),
              a => a,
            }),*,
            _    => Err(Err::Error(nom_ws_error_position!($i, $crate::lib::nom::ErrorKind::Switch)))
          }
        }
      }
    }
  );
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)*), $($rest:tt)*) => (
    {
      switch_sep!(__impl $i, $separator, $submac!($($args)*), $($rest)*)
    }
  );
  ($i:expr, $separator:path, $e:path, $($rest:tt)*) => (
    {
      switch_sep!(__impl $i, $separator, nom_call!($e), $($rest)*)
    }
  );
);

#[doc(hidden)]
#[cfg(feature = "alloc")]
#[macro_export(local_inner_macros)]
macro_rules! separated_list_sep (
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    separated_list!(
      $i,
      sep!($separator, $submac!($($args)*)),
      sep!($separator, $submac2!($($args2)*))
    )
  );
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* ), $g:expr) => (
    separated_list_sep!($i, $separator, $submac!($($args)*), nom_call!($g));
  );
  ($i:expr, $separator:path, $f:expr, $submac:ident!( $($args:tt)* )) => (
    separated_list_sep!($i, $separator, nom_call!($f), $submac!($($args)*));
  );
  ($i:expr, $separator:path, $f:expr, $g:expr) => (
    separated_list_sep!($i, $separator, nom_call!($f), nom_call!($g));
  );
);

/// helper macros to build a separator parser
///
/// ```
/// # #[macro_use] extern crate nom;
/// named!(pub space, eat_separator!(&b" \t"[..]));
/// # fn main() {}
/// ```
#[macro_export(local_inner_macros)]
macro_rules! eat_separator (
  ($i:expr, $arr:expr) => (
    {
      use $crate::lib::nom::{FindToken, InputTakeAtPosition};
      let input = $i;
      input.split_at_position(|c| !$arr.find_token(c))
    }
  );
);

/// sep is the parser rewriting macro for whitespace separated formats
///
/// it takes as argument a space eating function and a parser tree,
/// and will intersperse the space parser everywhere
///
/// ```ignore
/// #[macro_export(local_inner_macros)]
/// macro_rules! ws (
///   ($i:expr, $($args:tt)*) => (
///     {
///       use sp;
///       sep!($i, sp, $($args)*)
///     }
///   )
/// );
/// ```
#[macro_export(local_inner_macros)]
macro_rules! sep (
  ($i:expr,  $separator:path, tuple ! ($($rest:tt)*) ) => {
    tuple_sep!($i, $separator, (), $($rest)*)
  };
  ($i:expr,  $separator:path, pair ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      pair_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, delimited ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      delimited_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, separated_pair ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      separated_pair_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, preceded ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      preceded_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, terminated ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      terminated_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, do_parse ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      do_parse_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, permutation ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      permutation_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, alt ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      alt_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, alt_complete ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      alt_complete_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, switch ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      switch_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, separated_list ! ($($rest:tt)*) ) => {
    wrap_sep!($i,
      $separator,
      separated_list_sep!($separator, $($rest)*)
    )
  };
  ($i:expr,  $separator:path, many0 ! ($($rest:tt)*) ) => {
    many0!($i, wrap_sep!($separator, $($rest)*))
  };
  ($i:expr,  $separator:path, many1 ! ($($rest:tt)*) ) => {
    many1!($i, wrap_sep!($separator, $($rest)*))
  };
  ($i:expr, $separator:path, return_error!( $($args:tt)* )) => {
    return_error!($i, wrap_sep!($separator, $($args)*))
  };
//FIXME: missing separated_nonempty_list,
// many_till, many_m_n, count, count_fixed, fold_many0, fold_many1,
// fold_many_m_n
  ($i:expr, $separator:path, $submac:ident!( $($args:tt)* )) => {
    wrap_sep!($i, $separator, $submac!($($args)*))
  };
  ($i:expr, $separator:path, $f:expr) => {
    wrap_sep!($i, $separator, nom_call!($f))
  };
);

use lib::nom::IResult;
use lib::nom::{AsChar, FindToken, InputTakeAtPosition};
#[allow(unused_imports)]
pub fn sp<'a, T>(input: T) -> IResult<T, T>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar + Clone,
  &'a str: FindToken<<T as InputTakeAtPosition>::Item>,
{
  input.split_at_position(|item| {
    let c = item.clone().as_char();
    !(c == ' ' || c == '\t' || c == '\r' || c == '\n')
  })
  //this could be written as followed, but not using FindToken is faster
  //eat_separator!(input, " \t\r\n")
}

/// `ws!(I -> IResult<I,O>) => I -> IResult<I, O>`
///
/// transforms a parser to automatically consume
/// whitespace between each token. By default,
/// it takes the following characters: `" \t\r\n"`.
///
/// If you need a whitespace parser consuming a
/// different set of characters, you can make
/// your own by reusing the `sep!` combinator.
///
/// To use `ws!`, pass your parser as argument:
///
/// ```
/// # #[macro_use] extern crate nom;
/// # fn main() {
/// named!(tuple<&[u8], (&[u8], &[u8]) >,
///   ws!(tuple!( take!(3), tag!("de") ))
/// );
///
/// assert_eq!(
///   tuple(&b" \t abc de fg"[..]),
///  Ok((&b"fg"[..], (&b"abc"[..], &b"de"[..])))
/// );
/// # }
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! ws (
  ($i:expr, $($args:tt)*) => (
    {
      use $crate::sp;
      use $crate::lib::nom::Convert;
      use $crate::lib::nom::Err;
      use $crate::lib::std::result::Result::*;

      match sep!($i, sp, $($args)*) {
        Err(e) => Err(e),
        Ok((i1,o))    => {
          match (sp)(i1) {
            Err(e) => Err(Err::convert(e)),
            Ok((i2,_))    => Ok((i2, o))
          }
        }
      }
    }
  )
);

#[cfg(test)]
#[allow(dead_code)]
mod tests {
  #[cfg(feature = "alloc")]
  use lib::std::string::{String, ToString};
  use nom::{Err, IResult, Needed};
  use super::sp;
  use nom::ErrorKind;
  use nom::types::CompleteStr;

  #[test]
  fn spaaaaace() {
    assert_eq!(sp(&b" \t abc "[..]), Ok((&b"abc "[..], &b" \t "[..])));
  }

  #[test]
  fn tag() {
    named!(abc, ws!(tag!("abc")));

    assert_eq!(abc(&b" \t abc def"[..]), Ok((&b"def"[..], &b"abc"[..])));
  }

  #[test]
  fn pair() {
    named!(pair_2<&[u8], (&[u8], &[u8]) >,
      ws!(pair!( take!(3), tag!("de") ))
    );

    assert_eq!(
      pair_2(&b" \t abc de fg"[..]),
      Ok((&b"fg"[..], (&b"abc"[..], &b"de"[..])))
    );
  }

  #[test]
  fn preceded() {
    named!(prec<&[u8], &[u8] >,
      ws!(preceded!( take!(3), tag!("de") ))
    );

    assert_eq!(prec(&b" \t abc de fg"[..]), Ok((&b"fg"[..], &b"de"[..])));
  }

  #[test]
  fn terminated() {
    named!(term<&[u8], &[u8] >,
      ws!(terminated!( take!(3), tag!("de") ))
    );

    assert_eq!(term(&b" \t abc de fg"[..]), Ok((&b"fg"[..], &b"abc"[..])));
  }

  #[test]
  fn tuple() {
    //trace_macros!(true);
    named!(tuple_2<&[u8], (&[u8], &[u8]) >,
      ws!(tuple!( take!(3), tag!("de") ))
    );
    //trace_macros!(false);

    assert_eq!(
      tuple_2(&b" \t abc de fg"[..]),
      Ok((&b"fg"[..], (&b"abc"[..], &b"de"[..])))
    );
  }

  #[test]
  fn levels() {
    //trace_macros!(true);
    named!(level_2<&[u8], (&[u8], (&[u8], &[u8])) >,
      ws!(pair!(take!(3), tuple!( tag!("de"), tag!("fg ") )))
    );
    //trace_macros!(false);

    assert_eq!(
      level_2(&b" \t abc de fg \t hi "[..]),
      Ok((&b"hi "[..], (&b"abc"[..], (&b"de"[..], &b"fg "[..]))))
    );
  }

  #[test]
  fn do_parse() {
    fn ret_int1(i: &[u8]) -> IResult<&[u8], u8> {
      Ok((i, 1))
    };
    fn ret_int2(i: &[u8]) -> IResult<&[u8], u8> {
      Ok((i, 2))
    };

    //trace_macros!(true);
    named!(do_parser<&[u8], (u8, u8)>,
      ws!(do_parse!(
        tag!("abcd")       >>
        opt!(tag!("abcd")) >>
        aa: ret_int1       >>
        tag!("efgh")       >>
        bb: ret_int2       >>
        tag!("efgh")       >>
        (aa, bb)
      ))
    );

    //trace_macros!(false);

    assert_eq!(
      do_parser(&b"abcd abcd\tefghefghX"[..]),
      Ok((&b"X"[..], (1, 2)))
    );
    assert_eq!(
      do_parser(&b"abcd\tefgh      efgh X"[..]),
      Ok((&b"X"[..], (1, 2)))
    );
    assert_eq!(
      do_parser(&b"abcd  ab"[..]),
      Err(Err::Incomplete(Needed::Size(4)))
    );
    assert_eq!(
      do_parser(&b" abcd\tefgh\tef"[..]),
      Err(Err::Incomplete(Needed::Size(4)))
    );
  }

  #[test]
  fn permutation() {
    //trace_macros!(true);
    named!(
      perm<(&[u8], &[u8], &[u8])>,
      ws!(permutation!(tag!("abcd"), tag!("efg"), tag!("hi")))
    );
    //trace_macros!(false);

    let expected = (&b"abcd"[..], &b"efg"[..], &b"hi"[..]);

    let a = &b"abcd\tefg \thijk"[..];
    assert_eq!(perm(a), Ok((&b"jk"[..], expected)));
    let b = &b"  efg  \tabcdhi jk"[..];
    assert_eq!(perm(b), Ok((&b"jk"[..], expected)));
    let c = &b" hi   efg\tabcdjk"[..];
    assert_eq!(perm(c), Ok((&b"jk"[..], expected)));

    let d = &b"efg  xyzabcdefghi"[..];
    assert_eq!(
      perm(d),
      Err(Err::Error(nom_ws_error_node_position!(
        &b"efg  xyzabcdefghi"[..],
        ErrorKind::Permutation,
        nom_ws_error_position!(&b"  xyzabcdefghi"[..], ErrorKind::Permutation)
      )))
    );

    let e = &b" efg \tabc"[..];
    assert_eq!(perm(e), Err(Err::Incomplete(Needed::Size(4))));
  }

  #[cfg(feature = "alloc")]
  #[derive(Debug, Clone, PartialEq)]
  pub struct ErrorStr(String);

  #[cfg(feature = "alloc")]
  impl From<u32> for ErrorStr {
    fn from(i: u32) -> Self {
      ErrorStr(format!("custom error code: {}", i))
    }
  }

  #[cfg(feature = "alloc")]
  impl<'a> From<&'a str> for ErrorStr {
    fn from(i: &'a str) -> Self {
      ErrorStr(format!("custom error message: {}", i))
    }
  }

  #[cfg(feature = "alloc")]
  #[test]
  fn alt() {
    fn work(input: &[u8]) -> IResult<&[u8], &[u8], ErrorStr> {
      Ok((&b""[..], input))
    }

    #[allow(unused_variables)]
    fn dont_work(input: &[u8]) -> IResult<&[u8], &[u8], ErrorStr> {
      use Context;
      Err(Err::Error(Context::Code(
        &b""[..],
        ErrorKind::Custom(ErrorStr("abcd".to_string())),
      )))
    }

    fn work2(input: &[u8]) -> IResult<&[u8], &[u8], ErrorStr> {
      Ok((input, &b""[..]))
    }

    fn alt1(i: &[u8]) -> IResult<&[u8], &[u8], ErrorStr> {
      alt!(i, dont_work | dont_work)
    }
    fn alt2(i: &[u8]) -> IResult<&[u8], &[u8], ErrorStr> {
      alt!(i, dont_work | work)
    }
    fn alt3(i: &[u8]) -> IResult<&[u8], &[u8], ErrorStr> {
      alt!(i, dont_work | dont_work | work2 | dont_work)
    }

    let a = &b"\tabcd"[..];
    assert_eq!(
      alt1(a),
      Err(Err::Error(nom_ws_error_position!(a, ErrorKind::Alt::<ErrorStr>)))
    );
    assert_eq!(alt2(a), Ok((&b""[..], a)));
    assert_eq!(alt3(a), Ok((a, &b""[..])));

    named!(alt4<CompleteStr, CompleteStr>, ws!(alt!(tag!("abcd") | tag!("efgh"))));
    assert_eq!(
      alt4(CompleteStr("\tabcd")),
      Ok((CompleteStr(""), CompleteStr(r"abcd")))
    );
    assert_eq!(
      alt4(CompleteStr("  efgh ")),
      Ok((CompleteStr(""), CompleteStr("efgh")))
    );

    // test the alternative syntax
    named!(alt5<CompleteStr, bool>, ws!(alt!(tag!("abcd") => { |_| false } | tag!("efgh") => { |_| true })));
    assert_eq!(alt5(CompleteStr("\tabcd")), Ok((CompleteStr(""), false)));
    assert_eq!(alt5(CompleteStr("  efgh ")), Ok((CompleteStr(""), true)));
  }

  /*FIXME: alt_complete works, but ws will return Incomplete on end of input
  #[test]
  fn alt_complete() {
    named!(ac<&[u8], &[u8]>,
      ws!(alt_complete!(tag!("abcd") | tag!("ef") | tag!("ghi") | tag!("kl")))
    );

    let a = &b""[..];
    assert_eq!(ac(a), Err(Err::Error(nom_ws_error_position!(a, ErrorKind::Alt))));
    let a = &b" \tef "[..];
    assert_eq!(ac(a),Ok((&b""[..], &b"ef"[..])));
    let a = &b" cde"[..];
    assert_eq!(ac(a), Err(Err::Error(nom_ws_error_position!(&a[1..], ErrorKind::Alt))));
  }
  */

  #[allow(unused_variables)]
  #[test]
  fn switch() {
    named!(sw<CompleteStr,CompleteStr>,
      ws!(switch!(take!(4),
        CompleteStr("abcd") => take!(2) |
        CompleteStr("efgh") => take!(4)
      ))
    );

    let a = CompleteStr(" abcd ef gh");
    assert_eq!(sw(a), Ok((CompleteStr("gh"), CompleteStr("ef"))));

    let b = CompleteStr("\tefgh ijkl ");
    assert_eq!(sw(b), Ok((CompleteStr(""), CompleteStr("ijkl"))));
    let c = CompleteStr("afghijkl");
    assert_eq!(
      sw(c),
      Err(Err::Error(nom_ws_error_position!(
        CompleteStr("afghijkl"),
        ErrorKind::Switch
      )))
    );
  }

  named!(str_parse(&str) -> &str, ws!(tag!("test")));
  #[allow(unused_variables)]
  #[test]
  fn str_test() {
    assert_eq!(str_parse(" \n   test\t a\nb"), Ok(("a\nb", "test")));
  }

  // test whitespace parser generation for alt
  named!(space, tag!(" "));
  #[cfg(feature = "alloc")]
  named!(pipeline_statement<&[u8], ()>,
    ws!(
      do_parse!(
      tag!("pipeline") >>
      attributes: delimited!(char!('{'),
                             separated_list!(char!(','), alt!(
                               space |
                               space
                             )),
                             char!('}')) >>

      ({
        let _ = attributes;
        ()
      })
    )
  )
  );

  #[cfg(feature = "alloc")]
  named!(
    fail<&[u8]>,
    map!(many_till!(take!(1), ws!(tag!("."))), |(r, _)| r[0])
  );
}
