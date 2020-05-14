#[macro_export]
macro_rules! jikan_response_entity {
    ($vis_st:vis struct $src_name:ident $(<$T:ident>)? {
        $(
            $( #[$attr_meta:meta] )*
            $attr_vis:vis $attr_name:ident: $attr_type:ty
        ),* $(,)*
    }) => {
        #[derive(Deserialize, Debug)]
        $vis_st struct $src_name $(<$T>)? {
            request_hash: String,
            request_cached: bool,
            request_cache_expiry: u32,
            $(
            $( #[$attr_meta] )*
            $attr_vis $attr_name: $attr_type
            ),*
        }
    };
}

#[macro_export]
macro_rules! builder {
    ( $vis_st:vis struct $src_name:ident {
        $( $attr_vis:vis $attr_name:ident : $attr_type:ty ),* $(,)?
    })
    => {
        $vis_st struct $src_name {
            $( $attr_vis $attr_name : Option<$attr_type> ),*
        }

        impl $src_name {
            pub fn builder() -> $src_name {
                $src_name {
                    $(
                        $attr_name : None
                    ),*
                }
            }

            $(
                pub fn $attr_name(mut self, value: $attr_type) -> Self {
                    self.$attr_name = Some(value);
                    self
                }
            )*
        }
    }
}