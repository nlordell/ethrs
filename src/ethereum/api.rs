//! Module implementing declarative API generation for JSON RPC namespaces.

use super::Provider;
use crate::jsonrpc::ClientError;
use crate::transport::Transport;

/// Trait with methods shared amongst APIs, allowing for easy extension.
pub trait Api<T> {
    fn provider(&mut self) -> Provider<T>;
}

/// Type Definition for API errors.
pub type ApiError<T> = ClientError<<T as Transport>::Error>;

/// Define a JSON RPC namespace with support for Ethereum specific types.
#[macro_export]
macro_rules! api {
    ($(
        $(#[$attr:meta])*
        module $ns:ident [$(
            $(#[$subns_attr:meta])*
            $subns:ident => $subns_type:ident,
        )*] {$(
            $(#[$method_attr:meta])*
            $method_name:ident as $method:ident ($(
                $param:ident : $param_type:ty $([ $param_serde:ty ])?
            ),* $(,)?) -> $result_type:ty $([ $result_serde:ty ])?;
        )*}
    )*) => {$(
        $(#[$attr])*
        pub struct $ns<'a, T>(pub $crate::ethereum::Provider<'a, T>);

        impl<T> $ns<'_, T>
        where
            T: $crate::transport::Transport,
        {$(
            $(#[$subns_attr])*
            pub fn $subns(&mut self) -> $subns_type<T> {
                $subns_type(self.provider())
            }
        )* $(
            $(#[$method_attr])*
            pub async fn $method(&mut self, $(
                $param: $param_type,
            )*) -> Result<$result_type, $crate::ethereum::ApiError<T>> {
                let params = ($(
                    __api!(ser: $param ; $param_type $([ $param_serde ])*),
                )*);

                let result = <$result_type as $crate::encoding::Decode<
                    __api!(de: $result_type $([ $result_serde ])*),
                >>::decode(
                    self.provider()
                        .call(stringify!($method_name), params)
                        .await?
                );

                Ok(result)
            }
        )*}

        impl<T> $crate::ethereum::Api<T> for $ns<'_, T> {
            fn provider(&mut self) -> $crate::ethereum::Provider<T> {
                self.0.shared()
            }
        }
    )*};
}

/// Utility macro used by [`ethrs::ethereum::api`].
#[doc(hidden)]
#[macro_export]
macro_rules! __api {
    (ser: $value:expr ; $type:ty) => {
        $value
    };
    (ser: $value:expr ; $type:ty [ $serde:ty ]) => {
        <$serde>::from($value)
    };

    (de: $type:ty) => {
        $type
    };
    (de: $type:ty [ $serde:ty ]) => {
        $serde
    };
}

/// Macro for testing generated API code.
#[cfg(test)]
macro_rules! api_test {
    ($(
        $method_name:ident as $ns:ident :: $call:ident {$(
            ($($param:expr),*): $request:expr => $response:expr
        )*}
    )*) => {
        #[test]
        fn $call() {
            use $crate::transport::MockTransport;

            let mut transport = MockTransport::default();
            transport$(
                .expect_call(stringify!($method_name), $request, Ok($response))
            )*;

            let provider = Provider::new(transport);
            let mut api = <$ns>(provider);

            $(
                api.$call($($param),*);
            )*
        }
    };
}
