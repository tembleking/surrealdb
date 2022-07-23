macro_rules! map {
    ($($k:expr => $v:expr),* $(,)? $( => $x:expr )?) => {{
        let mut m = ::std::collections::BTreeMap::new();
        $(m.extend($x.iter().map(|(k, v)| (k.clone(), v.clone())));)?
        $(m.insert($k, $v);)+
        m
    }};
}

macro_rules! mrg {
    ($($m:expr, $x:expr)+) => {{
        $($m.extend($x.iter().map(|(k, v)| (k.clone(), v.clone())));)+
        $($m)+
    }};
}

macro_rules! get_cfg {
	($i:ident : $($s:expr),+) => (
		let $i = || { $( if cfg!($i=$s) { return $s; } );+ "unknown"};
	)
}
