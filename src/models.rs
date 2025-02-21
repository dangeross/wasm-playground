use crate::errors::WasmError;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::VectorIntoJsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub type WasmResult<T> = Result<T, WasmError>;

macro_rules! wasm_struct_wrapper {
    ($name:ident,$ty:ty) => {
        #[derive(serde::Deserialize, serde::Serialize, tsify_next::Tsify)]
        #[tsify(from_wasm_abi, into_wasm_abi)]
        #[serde(transparent, rename_all = "camelCase")]
        pub struct $name(pub $ty);

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                $name(value)
            }
        }

        impl VectorIntoJsValue for $name {
            fn vector_into_jsvalue(vector: Box<[$name]>) -> JsValue {
                wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
            }
        }
    };
}

wasm_struct_wrapper!(
    AesSuccessActionData,
    sdk_common::prelude::AesSuccessActionData
);
wasm_struct_wrapper!(
    AesSuccessActionDataDecrypted,
    sdk_common::prelude::AesSuccessActionDataDecrypted
);
wasm_struct_wrapper!(
    AesSuccessActionDataResult,
    sdk_common::prelude::AesSuccessActionDataResult
);
wasm_struct_wrapper!(Amount, sdk_common::prelude::Amount);
wasm_struct_wrapper!(BitcoinAddressData, sdk_common::prelude::BitcoinAddressData);
wasm_struct_wrapper!(CurrencyInfo, sdk_common::prelude::CurrencyInfo);
wasm_struct_wrapper!(FiatCurrency, sdk_common::prelude::FiatCurrency);
wasm_struct_wrapper!(InputType, sdk_common::prelude::InputType);
wasm_struct_wrapper!(LiquidAddressData, sdk_common::prelude::LiquidAddressData);
wasm_struct_wrapper!(LNInvoice, sdk_common::prelude::LNInvoice);
wasm_struct_wrapper!(LNOffer, sdk_common::prelude::LNOffer);
wasm_struct_wrapper!(LnOfferBlindedPath, sdk_common::prelude::LnOfferBlindedPath);
wasm_struct_wrapper!(
    LnUrlAuthRequestData,
    sdk_common::prelude::LnUrlAuthRequestData
);
wasm_struct_wrapper!(LnUrlErrorData, sdk_common::prelude::LnUrlErrorData);
wasm_struct_wrapper!(
    LnUrlCallbackStatus,
    sdk_common::prelude::LnUrlCallbackStatus
);
wasm_struct_wrapper!(LnUrlPayErrorData, sdk_common::prelude::LnUrlPayErrorData);
wasm_struct_wrapper!(LnUrlPayRequest, sdk_common::prelude::LnUrlPayRequest);
wasm_struct_wrapper!(
    LnUrlPayRequestData,
    sdk_common::prelude::LnUrlPayRequestData
);
wasm_struct_wrapper!(
    LnUrlPaySuccessData,
    sdk_common::prelude::LnUrlPaySuccessData
);
wasm_struct_wrapper!(
    LnUrlWithdrawResult,
    sdk_common::prelude::LnUrlWithdrawResult
);
wasm_struct_wrapper!(
    LnUrlWithdrawRequest,
    sdk_common::prelude::LnUrlWithdrawRequest
);
wasm_struct_wrapper!(
    LnUrlWithdrawRequestData,
    sdk_common::prelude::LnUrlWithdrawRequestData
);
wasm_struct_wrapper!(
    LnUrlWithdrawSuccessData,
    sdk_common::prelude::LnUrlWithdrawSuccessData
);
wasm_struct_wrapper!(LocaleOverrides, sdk_common::prelude::LocaleOverrides);
wasm_struct_wrapper!(LocalizedName, sdk_common::prelude::LocalizedName);
wasm_struct_wrapper!(
    MessageSuccessActionData,
    sdk_common::prelude::MessageSuccessActionData
);
wasm_struct_wrapper!(Network, sdk_common::prelude::Network);
wasm_struct_wrapper!(Rate, sdk_common::prelude::Rate);
wasm_struct_wrapper!(RouteHint, sdk_common::prelude::RouteHint);
wasm_struct_wrapper!(RouteHintHop, sdk_common::prelude::RouteHintHop);
wasm_struct_wrapper!(SuccessAction, sdk_common::prelude::SuccessAction);
wasm_struct_wrapper!(
    SuccessActionProcessed,
    sdk_common::prelude::SuccessActionProcessed
);
wasm_struct_wrapper!(Symbol, sdk_common::prelude::Symbol);
wasm_struct_wrapper!(
    UrlSuccessActionData,
    sdk_common::prelude::UrlSuccessActionData
);
