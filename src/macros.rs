#[macro_export]
macro_rules! map (
    // -----------------------------------------------------------------------
    // Inferred types, no bindings.
    {$($key:expr => $value:expr;)+} => {
        vec![
            $(
                ($key, $value),
            )+
        ].into_iter().collect::<HashMap::<_, _>>()
    };
    // Inferred types, no bindings.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Right bound, left unbound.

    (_, $vi:ident := $kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $vi = $vt;
            vec![
                $(
                    ($key, $value),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    (_, $vi:ident := Into $kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $vi = $vt;
            vec![
                $(
                    ($key.into(), $value),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    (_, $vi:ident := $kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $vi = $vt;
            vec![
                $(
                    ($key, $value.into()),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    (_, $vi:ident := Into $kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $vi = $vt;
            vec![
                $(
                    ($key.into() , $value.into()),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    // Right bound, left unbound.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Left bound, right unbound.

    ($ki:ident, _ := $kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            vec![
                $(
                    ($key, $value),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    ($ki:ident, _ := Into $kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            vec![
                $(
                    ($key.into(), $value),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    ($ki:ident, _ := $kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            vec![
                $(
                    ($key, $value.into()),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    ($ki:ident, _ := Into $kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            vec![
                $(
                    ($key.into() , $value.into()),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    // Left bound, right unbound.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Both Bound

    ($ki:ident, $vi:ident := $kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            type $vi = $vt;
            vec![
                $(
                    ($key, $value),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    ($ki:ident, $vi:ident := Into $kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            type $vi = $vt;
            vec![
                $(
                    ($key.into(), $value),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    ($ki:ident, $vi:ident := $kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            type $vi = $vt;
            vec![
                $(
                    ($key, $value.into()),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    ($ki:ident, $vi:ident := Into $kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        {
            type $ki = $kt;
            type $vi = $vt;
            vec![
                $(
                    ($key.into() , $value.into()),
                )+
            ].into_iter().collect::<HashMap<$kt, $vt>>()
        }
    };

    // Both Bound
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // With types, but no bindings.

    ($kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        vec![
            $(
                ($key, $value),
            )+
        ].into_iter().collect::<HashMap<$kt, $vt>>()
    };


    (Into $kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        vec![
            $(
                ($key.into(), $value.into()),
            )+
        ].into_iter().collect::<HashMap<$kt, $vt>>()
    };

    (Into $kt:ty, $vt:ty {$($key:expr => $value:expr;)+}) => {
        vec![
            $(
                ($key.into(), $value),
            )+
        ].into_iter().collect::<HashMap<$kt, $vt>>()
    };

    ($kt:ty, Into $vt:ty {$($key:expr => $value:expr;)+}) => {
        vec![
            $(
                ($key, $value.into()),
            )+
        ].into_iter().collect::<HashMap<$kt, $vt>>()
    };
);
