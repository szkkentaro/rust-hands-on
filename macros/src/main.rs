// Matching
macro_rules! foo {
    (
        x => $e:expr
    ) => (
        println!("mode X: {}", $e)
    );
}

// Repetition
macro_rules! o_O {
    (
        $(
            $x:expr; [ $($y:expr),* ]
        );*
    ) => (
        &[ $($($x + $y),*),* ]
    )
}

// Hygiene
macro_rules! five_times {
    (
        $x:expr
    ) => (
        5 * $x
    );
}

// pass variant name
macro_rules! bar {
    (
        $v:ident
    ) => (
        let $v = 3;
    );
}

// requrcive macro
macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
   }};
}

fn main() {
    foo!(x => 3);
    // compile error: No rules expected the token `y`
    // foo!(y => 3);

    let a: &[i32] = o_O!(10; [1,2,3];
                         20; [4,5,6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    // It returns 25 as 5 * (2 + 3), not 13 as 5 * 2 + 3
    println!("{}", five_times!(2 + 3));

    // pass variant name to macro
    bar!(x);
    println!("{}", x); // print 3

    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["Macros guide"]]
            body[h1["Macros are best!"]]
        ]);

    assert_eq!(
        out,
        "<html><head><title>Macros guide</title></head>\
         <body><h1>Macros are best!</h1></body></html>"
    );
}
