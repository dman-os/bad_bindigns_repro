
mod shared {

    wit_bindgen::generate!({
        path: "./wit2",
        async: true,
        with: {
            "wasmcloud:postgres/query@0.1.1-draft": generate,
            "wasmcloud:postgres/types@0.1.1-draft": generate,
        }
    });
}

mod wit {
    wit_bindgen::generate!({
        world: "app",
        async: true,
        with: {
            "wasmcloud:postgres/query@0.1.1-draft": crate::shared::wasmcloud::postgres::query,
            "wasmcloud:postgres/types@0.1.1-draft": crate::shared::wasmcloud::postgres::types,
            // FIXME: can't address whole module, forced to address each type here
            // "foo:bar/feat-one": crate::feat_one::exports::foo::bar::feat_one,
            "foo:bar/feat-one/input": crate::feat_one::Input,
            "foo:bar/feat-one/output": crate::feat_one::Output,
        }
    });
}

struct Component;

wit::export!(Component with_types_in wit);

impl wit::exports::foo::bar::ctx::Guest for Component {
    #[allow(async_fn_in_trait)]
    async fn init()  {
        let rows = shared::wasmcloud::postgres::query::query(
            r#"
        SELECT now();
            "#
            .into(),
            vec![
            ],
        ).await.expect("postgres err");

        println!("{rows:?}");
    }
}

impl wit::exports::foo::bar::feat_one::Guest for Component {
    type FeatOne = feat_one::FeatOne;
}


fn main() {
    println!("Hello, world!");
}

mod feat_one {
    wit_bindgen::generate!({
        world: "temp-feat-one",
    });

    pub struct FeatOne;

    pub struct Input {
        pub hi: String,
    }
    pub struct Output {
        pub hello: String,
    }

    // NOTE: the trait from the top generate needs to be implemented to satisfy
    // the guest here, not the locally generated trait `GuestFeatOne`
    impl crate::wit::exports::foo::bar::feat_one::GuestFeatOne for FeatOne {
        #[allow(async_fn_in_trait)]
        async fn action(&self, inp: Input) -> Output {
            Output {
                hello: inp.hi
            }
        }
    }
}
