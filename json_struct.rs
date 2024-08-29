
macro_rules! define_json_struct {

    ($vis:vis $struct_name:ident { $($field_name:ident: $field_type:ty),* $(,)? }) => {
        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        $vis struct $struct_name(serde_json::Value);

        impl std::ops::Deref for $struct_name {
            type Target = serde_json::Value;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                let mut value = serde_json::json!({});
                $(
                    let default_value: $field_type = Default::default();
                    value[stringify!($field_name)] = serde_json::json!(default_value);
                )*
                Self(value)
            }
        }

        impl Into<serde_json::Value> for $struct_name {
            fn into(self) -> serde_json::Value {
                self.0
            }
        }

        impl $struct_name {
            $(
                paste! {
                    /// Clone and fetch the value of the `$field_name` field.
                    pub fn [<clone_ $field_name>](&self) -> $field_type {
                        serde_json::from_value(self.0[stringify!($field_name)].clone()).unwrap()
                    }
                }

                paste! {
                    /// Update the value of the `$field_name` field.
                    pub fn [<update_ $field_name>](&mut self, value: &$field_type) {
                        self.0[stringify!($field_name)] = serde_json::json!(value);
                    }
                }
            )*
        }
    };
}
