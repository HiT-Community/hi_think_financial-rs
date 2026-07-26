pub mod types;
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

	/// 发送带查询参数的 GET 请求，校验统一响应信封并返回业务数据。
	///
	/// HTTP 请求失败、HTTP 状态码非成功或 API 业务状态码非零时返回错误。
	pub async fn request<T, Q>(&self, path: &str, req: Option<Q>) -> anyhow::Result<T>
	where
		T: DeserializeOwned,
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

		let res = response.json::<types::ApiResponse<T>>().await?;
		if res.code != 0 {
			anyhow::bail!(
				"API request failed: path={path}, code={}, message={}, request_id={}",
				res.code,
				res.message,
				res.request_id,
			)
		}

		res.data.ok_or_else(|| anyhow::anyhow!("API response missing data: path={path}"))
	}

	/// 按 thscode、ticker 或中英文名称检索标的，支持跨市场消歧和子串匹配。
	///
	/// 接口地址：`GET /api/meta/tickers/search`。
	///
	/// 接口文档：https://fuyao.aicubes.cn/docs/api-reference/ticker-search/
	///
	/// `limit` 省略时由服务端使用默认值 10。
	pub async fn ticker_search(
		&self,
		req: types::TickerSearchRequest,
	) -> anyhow::Result<types::TickerSearchResponse> {
		const PATH: &str = "api/meta/tickers/search";

		self.request(PATH, Some(req)).await
	}

	/// 按资产类别分页获取标的代码表。
	///
	/// 接口地址：`GET /api/meta/tickers/list`。
	pub async fn ticker_list(
		&self,
		req: types::TickerListRequest,
	) -> anyhow::Result<types::TickerListResponse> {
		const PATH: &str = "api/meta/tickers/list";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只基金的基本资料。
	///
	/// 接口地址：`GET /api/fund/profile/detail`。
	pub async fn fund_profile_detail(
		&self,
		req: types::FundProfileDetailRequest,
	) -> anyhow::Result<types::FundProfileDetailResponse> {
		const PATH: &str = "api/fund/profile/detail";

		self.request(PATH, Some(req)).await
	}

	/// 获取基金定期披露的重仓股票及其持仓占比。
	///
	/// 接口地址：`GET /api/fund/portfolio/holdings`。
	pub async fn fund_portfolio_holdings(
		&self,
		req: types::FundPortfolioHoldingsRequest,
	) -> anyhow::Result<types::FundPortfolioHoldingsResponse> {
		const PATH: &str = "api/fund/portfolio/holdings";

		self.request(PATH, Some(req)).await
	}

	/// 获取基金单位净值和复权净值序列。
	///
	/// 接口地址：`GET /api/fund/performance/nav`。
	pub async fn fund_performance_nav(
		&self,
		req: types::FundPerformanceNavRequest,
	) -> anyhow::Result<types::FundPerformanceNavResponse> {
		const PATH: &str = "api/fund/performance/nav";

		self.request(PATH, Some(req)).await
	}

	/// 获取基金近一月、近一年和成立以来等区间收益率。
	///
	/// 接口地址：`GET /api/fund/performance/returns`。
	pub async fn fund_performance_returns(
		&self,
		req: types::FundPerformanceReturnsRequest,
	) -> anyhow::Result<types::FundPerformanceReturnsResponse> {
		const PATH: &str = "api/fund/performance/returns";

		self.request(PATH, Some(req)).await
	}

	/// 获取基金机构、个人投资者与持有人结构。
	///
	/// 接口地址：`GET /api/fund/holders/detail`。
	pub async fn fund_holders_detail(
		&self,
		req: types::FundHoldersDetailRequest,
	) -> anyhow::Result<types::FundHoldersDetailResponse> {
		const PATH: &str = "api/fund/holders/detail";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 ETF 的最新行情快照。
	///
	/// 接口地址：`GET /api/fund/market/snapshot`。
	pub async fn fund_market_snapshot(
		&self,
		req: types::FundMarketSnapshotRequest,
	) -> anyhow::Result<types::FundMarketSnapshotResponse> {
		const PATH: &str = "api/fund/market/snapshot";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 ETF 的历史日 K 线。
	///
	/// 接口地址：`GET /api/fund/market/historical`。
	pub async fn fund_market_historical(
		&self,
		req: types::FundMarketHistoricalRequest,
	) -> anyhow::Result<types::FundMarketHistoricalResponse> {
		const PATH: &str = "api/fund/market/historical";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只或多只 A 股的最新行情快照，或按分页遍历全市场快照。
	///
	/// 接口地址：`GET /api/a-share/prices/snapshot`。
	pub async fn a_share_prices_snapshot(
		&self,
		req: types::ASharePricesSnapshotRequest,
	) -> anyhow::Result<types::PricesSnapshotResponse> {
		const PATH: &str = "api/a-share/prices/snapshot";

		self.request(PATH, Some(req)).await
	}

	/// 批量获取 A 股最新估值快照。
	///
	/// 接口地址：`GET /api/a-share/valuations/snapshot`。
	pub async fn a_share_valuations_snapshot(
		&self,
		req: types::AShareValuationsSnapshotRequest,
	) -> anyhow::Result<types::AShareValuationsSnapshotResponse> {
		const PATH: &str = "api/a-share/valuations/snapshot";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 A 股在指定时间区间内的历史日 K 线。
	/// 接口地址：`GET /api/a-share/prices/historical`。
	pub async fn a_share_prices_historical(
		&self,
		req: types::ASharePricesHistoricalRequest,
	) -> anyhow::Result<types::ASharePricesHistoricalResponse> {
		const PATH: &str = "api/a-share/prices/historical";

		self.request(PATH, Some(req)).await
	}

	/// 获取全市场近十年未复权日 K Parquet 文件的短时有效下载链接。
	///
	/// 接口地址：`GET /api/dump/market-dumps/daily-k/download-url`。
	pub async fn market_dump_daily_k_download_url(
		&self,
	) -> anyhow::Result<types::MarketDumpDownloadUrlResponse> {
		const PATH: &str = "api/dump/market-dumps/daily-k/download-url";

		self.request::<types::MarketDumpDownloadUrlResponse, ()>(PATH, None).await
	}

	/// 获取全市场最近十个交易日未复权日 K Parquet 文件的短时有效下载链接。
	///
	/// 接口地址：`GET /api/dump/market-dumps/daily-k-10d/download-url`。
	pub async fn market_dump_daily_k_10d_download_url(
		&self,
	) -> anyhow::Result<types::MarketDumpDownloadUrlResponse> {
		const PATH: &str = "api/dump/market-dumps/daily-k-10d/download-url";

		self.request::<types::MarketDumpDownloadUrlResponse, ()>(PATH, None).await
	}

	/// 获取全市场复权因子 Parquet 文件的短时有效下载链接。
	///
	/// 接口地址：`GET /api/dump/market-dumps/adjustment-factors/download-url`。
	pub async fn market_dump_adjustment_factors_download_url(
		&self,
	) -> anyhow::Result<types::MarketDumpDownloadUrlResponse> {
		const PATH: &str = "api/dump/market-dumps/adjustment-factors/download-url";

		self.request::<types::MarketDumpDownloadUrlResponse, ()>(PATH, None).await
	}

	/// 获取单只 A 股的现金分红和送股复权因子事件流。
	///
	/// 接口地址：`GET /api/a-share/corporate-actions/adjustment-factors`。
	pub async fn adjustment_factors(
		&self,
		req: types::AdjustmentFactorsRequest,
	) -> anyhow::Result<types::AdjustmentFactorsResponse> {
		const PATH: &str = "api/a-share/corporate-actions/adjustment-factors";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 A 股的整体合并利润表多期序列。
	///
	/// 接口地址：`GET /api/a-share/financials/income-statements`。
	pub async fn income_statements(
		&self,
		req: types::FinancialStatementsRequest,
	) -> anyhow::Result<types::FinancialStatementsResponse<types::IncomeStatementItem>> {
		const PATH: &str = "api/a-share/financials/income-statements";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 A 股的整体合并资产负债表多期序列。
	///
	/// 接口地址：`GET /api/a-share/financials/balance-sheets`。
	pub async fn balance_sheets(
		&self,
		req: types::FinancialStatementsRequest,
	) -> anyhow::Result<types::FinancialStatementsResponse<types::BalanceSheetItem>> {
		const PATH: &str = "api/a-share/financials/balance-sheets";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 A 股的整体合并现金流量表多期序列。
	///
	/// 接口地址：`GET /api/a-share/financials/cash-flow-statements`。
	pub async fn cash_flow_statements(
		&self,
		req: types::FinancialStatementsRequest,
	) -> anyhow::Result<types::FinancialStatementsResponse<types::CashFlowStatementItem>> {
		const PATH: &str = "api/a-share/financials/cash-flow-statements";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 A 股指定报告期的成长、盈利、偿债、营运和现金流指标。
	///
	/// 接口地址：`GET /api/a-share/financials/indicators`。
	pub async fn financial_indicators(
		&self,
		req: types::FinancialIndicatorsRequest,
	) -> anyhow::Result<types::FinancialIndicatorsResponse> {
		const PATH: &str = "api/a-share/financials/indicators";

		self.request(PATH, Some(req)).await
	}

	/// 获取从今日向前一年的 A 股交易日序列。
	///
	/// 接口地址：`GET /api/a-share/calendar/trading-days`。
	pub async fn trading_days(&self) -> anyhow::Result<types::TradingDaysResponse> {
		const PATH: &str = "api/a-share/calendar/trading-days";

		self.request::<types::TradingDaysResponse, ()>(PATH, None).await
	}

	/// 按标签获取同花顺指数列表。
	///
	/// 接口地址：`GET /api/a-share-index/catalog/ths-index-list`。
	pub async fn ths_index_list(
		&self,
		req: types::ThsIndexListRequest,
	) -> anyhow::Result<types::ThsIndexListResponse> {
		const PATH: &str = "api/a-share-index/catalog/ths-index-list";

		self.request(PATH, Some(req)).await
	}

	/// 获取指定同花顺指数或标准指数的当前成分股列表。
	///
	/// 接口地址：`GET /api/a-share-index/constituents/ths-stock-list`。
	pub async fn ths_index_constituents(
		&self,
		req: types::ThsIndexConstituentsRequest,
	) -> anyhow::Result<types::ThsIndexConstituentsResponse> {
		const PATH: &str = "api/a-share-index/constituents/ths-stock-list";

		self.request(PATH, Some(req)).await
	}

	/// 按代码列表获取指数、板块或行业指数的最新行情快照。
	///
	/// 接口地址：`GET /api/a-share-index/prices/snapshot`。
	pub async fn index_prices_snapshot(
		&self,
		req: types::IndexPricesSnapshotRequest,
	) -> anyhow::Result<types::PricesSnapshotResponse> {
		const PATH: &str = "api/a-share-index/prices/snapshot";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只指数、板块或行业指数的历史日 K 线。
	///
	/// 接口地址：`GET /api/a-share-index/prices/historical`。
	pub async fn index_prices_historical(
		&self,
		req: types::IndexPricesHistoricalRequest,
	) -> anyhow::Result<types::IndexPricesHistoricalResponse> {
		const PATH: &str = "api/a-share-index/prices/historical";

		self.request(PATH, Some(req)).await
	}

	/// 按交易日、分页和排序条件获取 A 股涨停及连板股票池。
	///
	/// 接口地址：`GET /api/a-share/special-data/limit-up-pool`。
	pub async fn limit_up_pool(
		&self,
		req: types::LimitUpPoolRequest,
	) -> anyhow::Result<types::LimitUpPoolResponse> {
		const PATH: &str = "api/a-share/special-data/limit-up-pool";

		self.request(PATH, Some(req)).await
	}

	/// 获取近三十个交易日的 A 股连板梯队矩阵。
	///
	/// 接口地址：`GET /api/a-share/special-data/limit-up-ladder`。
	pub async fn limit_up_ladder(&self) -> anyhow::Result<types::LimitUpLadderResponse> {
		const PATH: &str = "api/a-share/special-data/limit-up-ladder";

		self.request::<types::LimitUpLadderResponse, ()>(PATH, None).await
	}

	/// 获取 A 股热度排名飙升榜。
	///
	/// 接口地址：`GET /api/a-share/special-data/skyrocket-list`。
	pub async fn skyrocket_list(
		&self,
		req: types::HotRankingRequest,
	) -> anyhow::Result<types::HotRankingResponse> {
		const PATH: &str = "api/a-share/special-data/skyrocket-list";

		self.request(PATH, Some(req)).await
	}

	/// 获取 A 股热股榜单。
	///
	/// 接口地址：`GET /api/a-share/special-data/hot-stock-list`。
	pub async fn hot_stock_list(
		&self,
		req: types::HotRankingRequest,
	) -> anyhow::Result<types::HotRankingResponse> {
		const PATH: &str = "api/a-share/special-data/hot-stock-list";

		self.request(PATH, Some(req)).await
	}

	/// 获取指定自然日的 A 股历史热股排行。
	///
	/// 接口地址：`GET /api/a-share/special-data/hot-stock-list-history`。
	pub async fn hot_stock_list_history(
		&self,
		req: types::HotStockListHistoryRequest,
	) -> anyhow::Result<types::HotStockListHistoryResponse> {
		const PATH: &str = "api/a-share/special-data/hot-stock-list-history";

		self.request(PATH, Some(req)).await
	}

	/// 获取单只 A 股在指定日期区间内的热榜排名走势。
	///
	/// 接口地址：`GET /api/a-share/special-data/hot-stock-rank-trend`。
	pub async fn hot_stock_rank_trend(
		&self,
		req: types::HotStockRankTrendRequest,
	) -> anyhow::Result<types::HotStockRankTrendResponse> {
		const PATH: &str = "api/a-share/special-data/hot-stock-rank-trend";

		self.request(PATH, Some(req)).await
	}

	/// 获取当日个股异动原因列表，可按异动标签过滤。
	///
	/// 接口地址：`GET /api/a-share/special-data/anomaly-analysis-list`。
	pub async fn anomaly_analysis_list(
		&self,
		req: types::AnomalyAnalysisListRequest,
	) -> anyhow::Result<types::AnomalyAnalysisResponse> {
		const PATH: &str = "api/a-share/special-data/anomaly-analysis-list";

		self.request(PATH, Some(req)).await
	}

	/// 按同花顺代码批量获取当日个股异动原因。
	///
	/// 接口地址：`GET /api/a-share/special-data/anomaly-analysis-stock`。
	pub async fn anomaly_analysis_stock(
		&self,
		req: types::AnomalyAnalysisStockRequest,
	) -> anyhow::Result<types::AnomalyAnalysisResponse> {
		const PATH: &str = "api/a-share/special-data/anomaly-analysis-stock";

		self.request(PATH, Some(req)).await
	}

	/// 获取全部、机构或游资维度的 A 股龙虎榜榜单。
	///
	/// 接口地址：`GET /api/a-share/special-data/dragon-tiger-list`。
	pub async fn dragon_tiger_list(
		&self,
		req: types::DragonTigerListRequest,
	) -> anyhow::Result<types::DragonTigerListResponse> {
		const PATH: &str = "api/a-share/special-data/dragon-tiger-list";

		self.request(PATH, Some(req)).await
	}
}

#[cfg(test)]
mod tests {
	use super::{HiThinkFinancialClient, types};
	use std::{
		io::{Read, Write},
		net::TcpListener,
		thread,
		time::Duration,
	};

	fn spawn_server(response_body: String) -> (String, thread::JoinHandle<String>) {
		let listener = TcpListener::bind("127.0.0.1:0").unwrap();
		let address = listener.local_addr().unwrap();
		let server = thread::spawn(move || {
			let (mut stream, _) = listener.accept().unwrap();
			stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
			let mut request = Vec::new();
			let mut buffer = [0_u8; 1_024];

			loop {
				let read = stream.read(&mut buffer).unwrap();
				if read == 0 {
					break;
				}
				request.extend_from_slice(&buffer[..read]);
				if request.windows(4).any(|window| window == b"\r\n\r\n") {
					break;
				}
			}

			let response_head = format!(
				"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
				response_body.len()
			);
			stream.write_all(response_head.as_bytes()).unwrap();
			stream.write_all(response_body.as_bytes()).unwrap();

			String::from_utf8(request).unwrap()
		});

		(format!("http://{address}/"), server)
	}

	fn client_for(base_url: String) -> HiThinkFinancialClient {
		HiThinkFinancialClient {
			base_url,
			api_key: "test-api-key".to_string(),
			inner: reqwest::Client::builder().no_proxy().build().unwrap(),
		}
	}

	#[tokio::test]
	async fn ticker_list_sends_documented_query_and_api_key() {
		let response_body = r#"{
			"code": 0,
			"message": "success",
			"request_id": "request-id",
			"data": {
				"timestamp": 1716105600000,
				"item": [{
					"thscode": "600519.SH",
					"ticker": "600519",
					"name": "Moutai",
					"exchange": "SH",
					"asset_type": "a-share",
					"currency": "CNY"
				}]
			}
		}"#
		.to_string();
		let (base_url, server) = spawn_server(response_body);
		let client = client_for(base_url);

		let response = client
			.ticker_list(types::TickerListRequest {
				asset_type: Some(types::TickerAssetType::AShare),
				limit: Some(25),
				offset: Some(50),
			})
			.await
			.unwrap();
		let request = server.join().unwrap();

		assert_eq!(response.item[0].thscode, "600519.SH");
		assert!(request.starts_with(
			"GET /api/meta/tickers/list?asset_type=a-share&limit=25&offset=50 HTTP/1.1\r\n"
		));
		assert!(request.to_ascii_lowercase().contains("x-api-key: test-api-key\r\n"));
	}

	#[tokio::test]
	async fn valuation_snapshot_sends_documented_query_and_deserializes_response() {
		let response_body = r#"{
			"code": 0,
			"message": "success",
			"request_id": "request-id",
			"data": {
				"timestamp": 1784736000000,
				"total": 1,
				"item": [{
					"thscode": "600519.SH",
					"ticker": "600519",
					"name": "Moutai",
					"pe_ttm": 21.3567,
					"pe_mrq": 20.8841,
					"pb_mrq": null,
					"ps_ttm": 10.3284,
					"pcf_ttm": 19.7716
				}]
			}
		}"#
		.to_string();
		let (base_url, server) = spawn_server(response_body);
		let client = client_for(base_url);

		let response = client
			.a_share_valuations_snapshot(types::AShareValuationsSnapshotRequest {
				thscodes: "600519.SH,000001.SZ".to_string(),
			})
			.await
			.unwrap();
		let request = server.join().unwrap();

		assert_eq!(response.item[0].pe_ttm, Some(21.3567));
		assert_eq!(response.item[0].pb_mrq, None);
		assert!(request.starts_with(
			"GET /api/a-share/valuations/snapshot?thscodes=600519.SH%2C000001.SZ HTTP/1.1\r\n"
		));
		assert!(request.to_ascii_lowercase().contains("x-api-key: test-api-key\r\n"));
	}

	#[tokio::test]
	async fn ticker_list_returns_context_for_api_business_errors() {
		let response_body = r#"{
			"code": 1001,
			"message": "missing parameter",
			"request_id": "request-id",
			"data": { "timestamp": 1716105600000, "item": [] }
		}"#
		.to_string();
		let (base_url, server) = spawn_server(response_body);
		let client = client_for(base_url);

		let error = client
			.ticker_list(types::TickerListRequest { asset_type: None, limit: None, offset: None })
			.await
			.unwrap_err();
		let request = server.join().unwrap();
		let message = error.to_string();

		assert!(request.starts_with("GET /api/meta/tickers/list HTTP/1.1\r\n"));
		assert!(message.contains("path=api/meta/tickers/list"));
		assert!(message.contains("code=1001"));
		assert!(message.contains("request_id=request-id"));
	}

	#[tokio::test]
	async fn ticker_list_rejects_success_response_without_business_data() {
		let response_body = r#"{
			"code": 0,
			"message": "success",
			"request_id": "request-id",
			"data": null
		}"#
		.to_string();
		let (base_url, server) = spawn_server(response_body);
		let client = client_for(base_url);

		let error = client
			.ticker_list(types::TickerListRequest { asset_type: None, limit: None, offset: None })
			.await
			.unwrap_err();
		server.join().unwrap();

		assert_eq!(error.to_string(), "API response missing data: path=api/meta/tickers/list");
	}
}
