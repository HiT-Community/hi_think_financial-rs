pub mod types;
use crate::types::ApiResponse;
use serde::{Serialize, de::DeserializeOwned};

const DEFAULT_URL: &str = "https://fuyao.aicubes.cn";

/// 同花顺金融数据 API 客户端。
///
/// 客户端复用底层 HTTP 连接，并在每个请求中自动携带 `X-api-key` 请求头。
/// 通过 [`HiThinkFinancialClient::new`] 创建实例后，可调用各数据接口方法。
pub struct HiThinkFinancialClient {
	/// API 服务的基础地址。
	///
	/// 默认值为 `https://fuyao.aicubes.cn`，请求方法会在此地址后拼接具体接口路径。
	base_url: String,
	/// 用于 API 身份认证的密钥。
	///
	/// 该值仅用于构造请求头，不会在日志、错误消息或接口返回值中暴露。
	api_key: String,
	/// 可复用连接池、超时配置和传输状态的底层 HTTP 客户端。
	///
	/// 由 [`reqwest::Client`] 管理，应随 `HiThinkFinancialClient` 一同长期复用，避免为每次请求重复创建连接池。
	inner: reqwest::Client,
}

impl HiThinkFinancialClient {
	/// 使用 API Key 创建客户端。
	pub fn new(ak: &str) -> Self {
		HiThinkFinancialClient {
			base_url: DEFAULT_URL.to_string(),
			api_key: ak.to_string(),
			inner: reqwest::Client::new(),
		}
	}

	/// 发送带查询参数的 GET 请求，并反序列化统一响应信封。
	///
	/// HTTP 请求失败、HTTP 状态码非成功或 API 业务状态码非零时返回错误。
	async fn request<T, Q>(&self, path: &str, req: Option<Q>) -> anyhow::Result<ApiResponse<T>>
	where
		T: DeserializeOwned + std::fmt::Debug,
		Q: Serialize,
	{
		let mut url = reqwest::Url::parse(&self.base_url)?.join(path)?;
		let mut qs = String::new();
		if let Some(req) = req {
			qs = serde_urlencoded::to_string(req)?;
		}

		if !qs.is_empty() {
			url.set_query(Some(&qs));
		}

		let response = self
			.inner
			.get(url)
			.header("X-api-key", &self.api_key)
			.send()
			.await?
			.error_for_status()?;

		let res = response.json::<ApiResponse<T>>().await?;
		if res.code != 0 {
			anyhow::bail!(
				"API request failed: path={path}, code={}, message={}, request_id={}",
				res.code,
				res.message,
				res.request_id,
			)
		} else {
			Ok(res)
		}
	}

	/// 按 thscode、ticker 或中英文名称检索标的，支持跨市场消歧和子串匹配。
	///
	/// 接口文档：https://fuyao.aicubes.cn/docs/api-reference/ticker-search/
	///
	/// `limit` 省略时由服务端使用默认值 10。
	pub async fn ticker_search(
		&self,
		req: types::TickerSearchRequest,
	) -> anyhow::Result<ApiResponse<types::TickerSearchResponse>> {
		const PATH: &str = "api/meta/tickers/search";

		self.request(PATH, Some(req)).await
	}
}
