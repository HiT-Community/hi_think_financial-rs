use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// 同花顺金融数据 API 的统一响应信封。
#[derive(Deserialize, Debug)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
pub struct ApiResponse<T: DeserializeOwned> {
	/// 业务状态码；`0` 表示请求成功。
	pub code: i64,
	/// 状态码对应的描述信息。
	pub message: String,
	/// 请求的唯一标识，可用于定位服务端日志。
	pub request_id: String,
	/// 接口实际返回的数据。
	pub data: T,
}

/// 标的检索接口的资产类别。
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TickerAssetType {
	/// A 股。
	AShare,
	/// A 股指数。
	AShareIndex,
}

/// 标的检索请求参数。
///
/// 接口文档：https://fuyao.aicubes.cn/docs/api-reference/ticker-search/
#[derive(Debug, Serialize)]
pub struct TickerSearchRequest {
	/// 搜索关键词，支持完整 thscode、ticker 代码或中英文名称的子串匹配。
	pub q: String,
	/// 可选的资产类别过滤条件。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_type: Option<TickerAssetType>,
	/// 可选的返回条数上限；服务端默认值为 10，最大值为 50。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
}

impl TickerSearchRequest {
	/// 使用必填的搜索关键词创建请求，保留服务端的可选参数默认值。
	pub fn new(q: impl Into<String>) -> Self {
		Self { q: q.into(), asset_type: None, limit: None }
	}
}

/// 标的检索接口的响应数据。
///
/// 接口文档：https://fuyao.aicubes.cn/docs/api-reference/ticker-search/
#[derive(Debug, Deserialize)]
pub struct TickerSearchResponse {
	/// 当前代码表快照的上游加载时间，单位为毫秒。
	pub timestamp: i64,
	/// 匹配到的标的列表。
	pub item: Vec<TickerSearchItem>,
}

/// 标的检索结果中的单个标的。
#[derive(Debug, Deserialize)]
pub struct TickerSearchItem {
	/// 完整的同花顺代码，例如 `600519.SH`。
	pub thscode: String,
	/// 不含交易所后缀的证券代码，例如 `600519`。
	pub ticker: String,
	/// 标的展示名称。
	pub name: String,
	/// 交易所后缀；无后缀的指数会返回 `null`。
	pub exchange: Option<String>,
	/// 对外资产类别。
	pub asset_type: TickerAssetType,
	/// 币种代码，A 股当前固定为 `CNY`。
	pub currency: String,
}

#[cfg(test)]
mod tests {
	use super::{TickerAssetType, TickerSearchRequest, TickerSearchResponse};

	#[test]
	fn serializes_only_required_search_parameters() {
		let request = TickerSearchRequest::new("中国平安");

		assert_eq!(
			serde_urlencoded::to_string(request).unwrap(),
			"q=%E4%B8%AD%E5%9B%BD%E5%B9%B3%E5%AE%89"
		);
	}

	#[test]
	fn serializes_optional_search_parameters() {
		let request = TickerSearchRequest {
			q: "000001".to_string(),
			asset_type: Some(TickerAssetType::AShareIndex),
			limit: Some(50),
		};

		assert_eq!(
			serde_urlencoded::to_string(request).unwrap(),
			"q=000001&asset_type=a-share-index&limit=50"
		);
	}

	#[test]
	fn deserializes_all_ticker_fields() {
		let response: TickerSearchResponse = serde_json::from_str(
			r#"{
				"timestamp": 1716105600000,
				"item": [{
					"thscode": "000001.SH",
					"ticker": "000001",
					"name": "上证指数",
					"exchange": null,
					"asset_type": "a-share-index",
					"currency": "CNY"
				}]
			}"#,
		)
		.unwrap();

		let item = &response.item[0];
		assert_eq!(response.timestamp, 1_716_105_600_000);
		assert_eq!(item.thscode, "000001.SH");
		assert_eq!(item.exchange, None);
		assert_eq!(item.asset_type, TickerAssetType::AShareIndex);
	}
}
