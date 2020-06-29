macro_rules! node {
    (struct $name:ident = $rule:path) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pub value: String,
        }

        impl $crate::ast::Node for $name {
            fn can_parse(rule: &Rule) -> bool {
                rule == &$rule
            }

            fn parse(pair: $crate::ast::Pair<Rule>) -> $crate::error::ParseResult<Self> {
                $crate::ast::helpers::check_rule(&pair, &$rule)?;
                let value = pair.as_str().to_string();
                Ok(Self { value })
            }
        }
    };

    (struct $name:ident = $rule:path {
        $(
            $(#[$meta:meta])* $field:ident: $field_type:ty
        ),* $(,)?
    }) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            $(
                pub $field: $field_type,
            )*
        }

        impl $crate::ast::Node for $name {
            fn can_parse(rule: &Rule) -> bool {
                rule == &$rule
            }

            fn parse(pair: $crate::ast::Pair<Rule>) -> $crate::error::ParseResult<Self> {
                $crate::ast::helpers::check_rule(&pair, &$rule)?;
                let mut pairs = pair.into_inner();

                Ok(Self {
                    $(
                        $field: node!(@parse_many $(#[$meta])* (pairs))?
                    ),*
                })
            }
        }
    };

    (enum $name:ident = $rule:path {
        $(
            $(#[$meta:meta])* $variant:ident($variant_type:ty)
        ),* $(,)?
    }) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $(
                $variant($variant_type),
            )*
        }

        impl $crate::ast::Node for $name {
            fn can_parse(rule: &Rule) -> bool {
                rule == &$rule
            }

            fn parse(pair: $crate::ast::Pair<Rule>) -> $crate::error::ParseResult<Self> {
                $crate::ast::helpers::check_rule(&pair, &$rule)?;

                let inner = $crate::ast::helpers::into_single(pair.into_inner())?;
                $(
                    if <$variant_type as $crate::ast::Node>::can_parse(&inner.as_rule()) {
                        node!(@parse $(#[$meta])* (inner)).map(Self::$variant)
                    } else
                )*
                {
                    unreachable!("rule {:?}", inner.as_rule())
                }
            }
        }
    };

    (@parse ($pair:expr)) => { $crate::ast::Node::parse($pair) };
    (@parse_many ($pairs:expr)) => { $crate::ast::Node::parse_many(&mut $pairs) };
}
