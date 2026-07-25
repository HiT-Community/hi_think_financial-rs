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
	pub data: Option<T>,
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
/// 接口地址：`GET /api/meta/tickers/search`。
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
/// 接口地址：`GET /api/meta/tickers/search`。
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

/// 标的列表获取接口的请求参数。
///
/// 接口地址：`GET /api/meta/tickers/list`。
#[derive(Debug, Serialize)]
pub struct TickerListRequest {
	/// 可选的资产类别；省略时服务端默认返回 A 股。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub asset_type: Option<TickerAssetType>,
	/// 可选的单页返回条数；服务端默认值为 1000，最大为 10000。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u16>,
	/// 可选的分页偏移量；服务端默认值为 0。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u64>,
}

/// 标的列表获取接口的响应数据。
///
/// 接口地址：`GET /api/meta/tickers/list`。
pub type TickerListResponse = TickerSearchResponse;

/// A 股行情的 K 线周期。
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PriceInterval {
	/// 日 K，序列化为 `1d`。
	#[serde(rename = "1d")]
	Day1,
}

/// A 股历史 K 线的复权方式。
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PriceAdjust {
	/// 不复权。
	None,
	/// 前复权。
	Forward,
	/// 后复权。
	Backward,
}

/// A 股行情快照接口的请求参数。
///
/// 接口地址：`GET /api/a-share/prices/snapshot`。
#[derive(Debug, Serialize)]
pub struct ASharePricesSnapshotRequest {
	/// 可选的逗号分隔同花顺代码列表；提供后服务端忽略分页参数。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thscodes: Option<String>,
	/// 可选的全市场分页大小；省略时服务端默认值为 100。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u16>,
	/// 可选的全市场分页偏移量；省略时服务端默认值为 0。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u64>,
}

/// A 股估值快照接口的请求参数。
///
/// 接口地址：`GET /api/a-share/valuations/snapshot`。
#[derive(Debug, Serialize)]
pub struct AShareValuationsSnapshotRequest {
	/// 以英文逗号分隔的 A 股完整同花顺代码列表。
	pub thscodes: String,
}

/// A 股估值快照接口的响应数据。
///
/// 接口地址：`GET /api/a-share/valuations/snapshot`。
#[derive(Debug, Deserialize)]
pub struct AShareValuationsSnapshotResponse {
	/// 数据就绪时间（毫秒）。
	pub timestamp: i64,
	/// 匹配到的股票数量。
	pub total: i64,
	/// 估值快照记录列表。
	pub item: Vec<AShareValuationSnapshotItem>,
}

/// 单条 A 股估值快照记录。
#[derive(Debug, Deserialize)]
pub struct AShareValuationSnapshotItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 不含交易所后缀的证券代码。
	pub ticker: String,
	/// 证券名称。
	pub name: String,
	/// 滚动市盈率。
	pub pe_ttm: Option<f64>,
	/// 单季度市盈率。
	pub pe_mrq: Option<f64>,
	/// 市净率。
	pub pb_mrq: Option<f64>,
	/// 滚动市销率。
	pub ps_ttm: Option<f64>,
	/// 滚动市现率。
	pub pcf_ttm: Option<f64>,
}

/// A 股或指数行情快照接口的响应数据。
/// 接口地址：`GET /api/a-share/prices/snapshot` 或
/// `GET /api/a-share-index/prices/snapshot`。
#[derive(Debug, Deserialize)]
pub struct PricesSnapshotResponse {
	/// 数据就绪时间（毫秒）；按显式代码列表查询时服务端返回 `null`。
	pub timestamp: Option<i64>,
	/// 全市场代码表总数，或显式请求的代码数量。
	pub total: i64,
	/// 行情快照记录列表。
	pub item: Vec<PriceSnapshotItem>,
}

/// 单条行情快照记录。
#[derive(Debug, Deserialize)]
pub struct PriceSnapshotItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 不含交易所后缀的证券代码。
	pub ticker: String,
	/// 最新成交价，单位为原始货币。
	pub last_price: f64,
	/// 相对前收盘价的涨跌额，单位为原始货币。
	pub price_change: f64,
	/// 涨跌幅百分比数值，例如 `1.74` 表示上涨 1.74%。
	pub price_change_ratio_pct: f64,
	/// 当日开盘价。
	pub open_price: f64,
	/// 当日最高价。
	pub high_price: f64,
	/// 当日最低价。
	pub low_price: f64,
	/// 前收盘价。
	pub prev_price: f64,
	/// 成交量，单位为股。
	pub volume: f64,
	/// 成交额，单位为原始货币。
	pub turnover: f64,
}

/// A 股历史 K 线接口的请求参数。
///
/// 接口地址：`GET /api/a-share/prices/historical`。
#[derive(Debug, Serialize)]
pub struct ASharePricesHistoricalRequest {
	/// 单只带交易所后缀的同花顺代码；不支持逗号分隔的多标的请求。
	pub thscode: String,
	/// K 线周期；当前服务端仅支持日 K。
	pub interval: PriceInterval,
	/// 查询起始时间，Unix 毫秒时间戳。
	pub start: i64,
	/// 查询结束时间，Unix 毫秒时间戳。
	pub end: i64,
	/// 可选的复权方式；省略时服务端默认前复权。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub adjust: Option<PriceAdjust>,
	/// 可选的分页偏移量；省略时服务端默认值为 0。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u64>,
}

/// A 股历史 K 线接口的响应数据。
///
/// 接口地址：`GET /api/a-share/prices/historical`。
#[derive(Debug, Deserialize)]
pub struct ASharePricesHistoricalResponse {
	/// 数据就绪时间，为序列中最新 K 线的上游有效时间，单位为毫秒。
	pub timestamp: i64,
	/// K 线记录列表。
	pub item: Vec<PriceBarItem>,
}

/// 单条日 K 线记录。
#[derive(Debug, Deserialize)]
pub struct PriceBarItem {
	/// K 线日期，Unix 毫秒时间戳。
	pub date_ms: i64,
	/// 开盘价。
	pub open_price: f64,
	/// 最高价。
	pub high_price: f64,
	/// 最低价。
	pub low_price: f64,
	/// 收盘价。
	pub close_price: f64,
	/// 成交量，单位为股。
	pub volume: f64,
	/// 成交额，单位为原始货币。
	pub turnover: f64,
}

/// 市场数据导出接口的响应数据。
///
/// 三个导出接口均返回短时有效的 S3 预签名下载链接：
/// `GET /api/dump/market-dumps/daily-k/download-url`、
/// `GET /api/dump/market-dumps/daily-k-10d/download-url` 与
/// `GET /api/dump/market-dumps/adjustment-factors/download-url`。
#[derive(Debug, Deserialize)]
pub struct MarketDumpDownloadUrlResponse {
	/// 指向 Parquet 文件的短时有效预签名下载链接。
	pub presigned_url: String,
	/// 预签名下载链接的过期时间。
	pub presigned_url_expires_at: String,
	/// 预签名下载链接距离过期的秒数。
	pub expires_in_seconds: u64,
}

/// A 股复权因子事件流接口的请求参数。
///
/// 接口地址：`GET /api/a-share/corporate-actions/adjustment-factors`。
#[derive(Debug, Serialize)]
pub struct AdjustmentFactorsRequest {
	/// 单只带交易所后缀的同花顺代码。
	pub thscode: String,
	/// 可选的事件起始日，格式为 `YYYY-MM-DD`。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub from: Option<String>,
	/// 可选的事件截止日，格式为 `YYYY-MM-DD`。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub to: Option<String>,
}

/// A 股复权因子事件流接口的响应数据。
///
/// 接口地址：`GET /api/a-share/corporate-actions/adjustment-factors`。
#[derive(Debug, Deserialize)]
pub struct AdjustmentFactorsResponse {
	/// 本次返回所属标的的完整同花顺代码。
	pub thscode: String,
	/// 本次返回所属标的的纯证券代码。
	pub ticker: String,
	/// 按除权除息日降序排列的事件列表。
	pub item: Vec<AdjustmentFactorItem>,
}

/// 单条复权因子事件记录。
#[derive(Debug, Deserialize)]
pub struct AdjustmentFactorItem {
	/// 不含交易所后缀的证券代码。
	pub ticker: String,
	/// 除权除息日，Asia/Shanghai 零点的 Unix 毫秒时间戳。
	pub ex_date_ms: i64,
	/// 每股现金分红（税前，原始货币）；非现金事件为 0。
	pub dividend_per_share: f64,
	/// 每股送股比例，例如 `0.1` 表示每 10 股送 1 股。
	pub per_share_bonus: f64,
}

/// 财务报表的报告期类型。
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FinancialPeriod {
	/// 年报，仅包含第四季度报告期。
	Annual,
	/// 季报，包含每个季度末报告期。
	Quarterly,
}

/// 三张财务报表共用的请求参数。
///
/// 接口地址：`GET /api/a-share/financials/income-statements`、
/// `GET /api/a-share/financials/balance-sheets` 与
/// `GET /api/a-share/financials/cash-flow-statements`。
#[derive(Debug, Serialize)]
pub struct FinancialStatementsRequest {
	/// 单只带交易所后缀的同花顺代码。
	pub thscode: String,
	/// 报告期类型。
	pub period: FinancialPeriod,
	/// 可选的最近报告期数；与 `start`、`end` 互斥，服务端默认值为 4。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u8>,
	/// 可选的时间区间起点，Unix 毫秒时间戳；必须与 `end` 同时提供。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start: Option<i64>,
	/// 可选的时间区间终点，Unix 毫秒时间戳；必须与 `start` 同时提供。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end: Option<i64>,
}

/// 财务报表共用的响应容器。
#[derive(Debug, Deserialize)]
pub struct FinancialStatementsResponse<T> {
	/// 本批报表中最大的报告期末时间，单位为毫秒。
	pub timestamp: i64,
	/// 报表记录列表，按报告期末降序排列。
	pub item: Vec<T>,
}

/// 三张财务报表记录共用的元数据。
#[derive(Debug, Deserialize)]
pub struct FinancialStatementMetadata {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 不含交易所后缀的证券代码。
	pub ticker: String,
	/// 请求报告期类型的回显值。
	pub period: FinancialPeriod,
	/// 财年，自然年。
	pub fiscal_year: i64,
	/// 财务报告季度标识：`FY`、`Q1`、`Q2`、`Q3` 或 `Q4`。
	pub fiscal_period: String,
	/// 披露日，Unix 毫秒时间戳。
	pub report_date_ms: i64,
	/// 报告期末，Asia/Shanghai 零点的 Unix 毫秒时间戳。
	pub period_end_ms: i64,
	/// 币种代码，A 股为 `CNY`。
	pub currency: String,
}

/// 利润表记录。
#[derive(Debug, Deserialize)]
pub struct IncomeStatementItem {
	/// 本期利润表的通用报告期元数据。
	#[serde(flatten)]
	pub metadata: FinancialStatementMetadata,
	/// 营业收入，原币元；未披露时为 `null`。
	pub operating_income: Option<f64>,
	/// 营业成本，原币元；未披露时为 `null`。
	pub operating_costs: Option<f64>,
	/// 营业总成本，原币元；未披露时为 `null`。
	pub operating_expenses: Option<f64>,
	/// 销售费用，原币元；未披露时为 `null`。
	pub sales_fee: Option<f64>,
	/// 管理费用，原币元；未披露时为 `null`。
	pub manage_fee: Option<f64>,
	/// 研发费用，原币元；未披露时为 `null`。
	pub research_and_development_expenses: Option<f64>,
	/// 营业利润，原币元；未披露时为 `null`。
	pub operating_profit: Option<f64>,
	/// 利息费用，原币元；未披露时为 `null`。
	pub interest_expenses: Option<f64>,
	/// 利润总额，原币元；未披露时为 `null`。
	pub profit_total: Option<f64>,
	/// 所得税费用，原币元；未披露时为 `null`。
	pub income_tax_expense: Option<f64>,
	/// 净利润，原币元；未披露时为 `null`。
	pub net_profit: Option<f64>,
	/// 归属于母公司股东的净利润，原币元；未披露时为 `null`。
	pub parent_holder_net_profit: Option<f64>,
	/// 基本每股收益，元每股；未披露时为 `null`。
	pub basic_eps: Option<f64>,
}

/// 资产负债表记录。
#[derive(Debug, Deserialize)]
pub struct BalanceSheetItem {
	/// 本期资产负债表的通用报告期元数据。
	#[serde(flatten)]
	pub metadata: FinancialStatementMetadata,
	/// 资产总计，原币元；未披露时为 `null`。
	pub assets_total: Option<f64>,
	/// 流动资产合计，原币元；未披露时为 `null`。
	pub total_current_assets: Option<f64>,
	/// 非流动资产合计，原币元；未披露时为 `null`。
	pub non_current_nets_total: Option<f64>,
	/// 货币资金，原币元；未披露时为 `null`。
	pub cash: Option<f64>,
	/// 应收账款，原币元；未披露时为 `null`。
	pub accounts_receivable: Option<f64>,
	/// 负债合计，原币元；未披露时为 `null`。
	pub total_debt: Option<f64>,
	/// 所有者权益（股东权益）合计，原币元；未披露时为 `null`。
	pub holder_equity_total: Option<f64>,
}

/// 现金流量表记录。
#[derive(Debug, Deserialize)]
pub struct CashFlowStatementItem {
	/// 本期现金流量表的通用报告期元数据。
	#[serde(flatten)]
	pub metadata: FinancialStatementMetadata,
	/// 经营活动产生的现金流量净额，原币元；未披露时为 `null`。
	pub act_cash_flow_net: Option<f64>,
	/// 投资活动产生的现金流量净额，原币元；未披露时为 `null`。
	pub invest_cash_flow_net: Option<f64>,
	/// 筹资活动产生的现金流量净额，原币元；未披露时为 `null`。
	pub financing_cash_flow_net: Option<f64>,
	/// 购建固定资产、无形资产和其他长期资产支付的现金，原币元；未披露时为 `null`。
	pub pay_fixed_assets_etc_cash: Option<f64>,
	/// 分配股利、利润或偿付利息支付的现金，原币元；未披露时为 `null`。
	pub pay_dividends_profits_interest_cash: Option<f64>,
	/// 现金及现金等价物净增加额，原币元；未披露时为 `null`。
	pub cash_equivalents_net_addition: Option<f64>,
}

/// 财务指标接口的请求参数。
///
/// 接口地址：`GET /api/a-share/financials/indicators`。
#[derive(Debug, Serialize)]
pub struct FinancialIndicatorsRequest {
	/// 单只带交易所后缀的同花顺代码。
	pub thscode: String,
	/// 报告期，格式为 `yyyy-1`、`yyyy-2`、`yyyy-3` 或 `yyyy-4`。
	pub report: String,
}

/// 财务指标接口的响应数据。
///
/// 接口地址：`GET /api/a-share/financials/indicators`。
#[derive(Debug, Deserialize)]
pub struct FinancialIndicatorsResponse {
	/// 请求同花顺代码的回显值。
	pub thscode: String,
	/// 请求报告期的回显值。
	pub report: String,
	/// 成长、盈利、偿债、营运和现金流五类能力指标块。
	pub abilities: Vec<FinancialAbility>,
}

/// 一类财务能力指标。
#[derive(Debug, Deserialize)]
pub struct FinancialAbility {
	/// 能力标识，例如 `growth`、`profitability` 或 `cash-flow`。
	pub ability: String,
	/// 当前能力下的指标列表。
	pub indicators: Vec<FinancialIndicator>,
}

/// 单个财务指标值。
#[derive(Debug, Deserialize)]
pub struct FinancialIndicator {
	/// 指标 ID，例如 `total_assets_growth_ratio`。
	pub index_id: String,
	/// 指标原始数值字符串；上游缺失时为 `null`。
	pub value: Option<String>,
}

/// A 股交易日历接口的响应数据。
///
/// 接口地址：`GET /api/a-share/calendar/trading-days`。
#[derive(Debug, Deserialize)]
pub struct TradingDaysResponse {
	/// 数据就绪时间，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 按时间升序排列的交易日列表。
	pub item: Vec<TradingDayItem>,
}

/// 单个 A 股交易日。
#[derive(Debug, Deserialize)]
pub struct TradingDayItem {
	/// Asia/Shanghai 零点的 Unix 毫秒时间戳。
	pub date_ms: i64,
	/// `yyyyMMdd` 格式的可读交易日。
	pub date: String,
}

/// 同花顺指数目录标签。
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ThsIndexTag {
	/// A 股概念指数。
	#[serde(rename = "cn_concept")]
	CnConcept,
	/// 区域指数。
	Region,
	/// 特色指数。
	Tszs,
	/// 行业指数。
	Industry,
}

/// 同花顺指数列表接口的请求参数。
///
/// 接口地址：`GET /api/a-share-index/catalog/ths-index-list`。
#[derive(Debug, Serialize)]
pub struct ThsIndexListRequest {
	/// 可选的指数标签；省略时服务端默认返回概念指数。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag: Option<ThsIndexTag>,
}

/// 同花顺指数列表接口的响应数据。
///
/// 接口地址：`GET /api/a-share-index/catalog/ths-index-list`。
#[derive(Debug, Deserialize)]
pub struct ThsIndexListResponse {
	/// 数据时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 同花顺指数列表。
	pub item: Vec<ThsIndexItem>,
}

/// 单个同花顺指数目录项。
#[derive(Debug, Deserialize)]
pub struct ThsIndexItem {
	/// 同花顺指数的完整同花顺代码。
	pub thscode: String,
	/// 同花顺指数展示名称。
	pub name: String,
}

/// 同花顺指数成分股接口的请求参数。
///
/// 接口地址：`GET /api/a-share-index/constituents/ths-stock-list`。
#[derive(Debug, Serialize)]
pub struct ThsIndexConstituentsRequest {
	/// 单只指数的完整同花顺代码。
	pub thscode: String,
}

/// 同花顺指数成分股接口的响应数据。
///
/// 接口地址：`GET /api/a-share-index/constituents/ths-stock-list`。
#[derive(Debug, Deserialize)]
pub struct ThsIndexConstituentsResponse {
	/// 数据时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 当前成分股列表。
	pub item: Vec<ThsIndexConstituentItem>,
}

/// 单个同花顺指数成分股。
#[derive(Debug, Deserialize)]
pub struct ThsIndexConstituentItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 不含交易所后缀的证券代码。
	pub ticker: String,
	/// 成分股展示名称。
	pub name: String,
}

/// 指数行情快照接口的请求参数。
///
/// 接口地址：`GET /api/a-share-index/prices/snapshot`。
#[derive(Debug, Serialize)]
pub struct IndexPricesSnapshotRequest {
	/// 必填的逗号分隔指数同花顺代码列表。
	pub thscodes: String,
	/// 为签名对齐保留的可选分页大小；服务端当前不使用。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u16>,
	/// 为签名对齐保留的可选分页偏移量；服务端当前不使用。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub offset: Option<u64>,
}

/// 指数历史 K 线接口的请求参数。
///
/// 接口地址：`GET /api/a-share-index/prices/historical`。
#[derive(Debug, Serialize)]
pub struct IndexPricesHistoricalRequest {
	/// 单只指数的完整同花顺代码。
	pub thscode: String,
	/// K 线周期；当前服务端仅支持日 K。
	pub interval: PriceInterval,
	/// 查询起始时间，Unix 毫秒时间戳。
	pub start: i64,
	/// 查询结束时间，Unix 毫秒时间戳。
	pub end: i64,
}

/// 指数历史 K 线接口的响应数据。
///
/// 接口地址：`GET /api/a-share-index/prices/historical`。
#[derive(Debug, Deserialize)]
pub struct IndexPricesHistoricalResponse {
	/// 数据就绪时间，为序列中最新 K 线的上游有效时间，单位为毫秒。
	pub timestamp: i64,
	/// 指数不存在复权语义，服务端固定返回 `null`。
	pub adjust: Option<PriceAdjust>,
	/// K 线记录列表。
	pub item: Vec<PriceBarItem>,
}

/// 涨停股票池的排序字段。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitUpSortField {
	/// 按最新价排序。
	LastPrice,
	/// 按连板数排序。
	ContinueDayCnt,
	/// 按当前封单额排序。
	SealMoney,
	/// 按涨停时间排序。
	LimitUpTime,
}

/// 涨停股票池的排序方向。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
	/// 升序。
	Asc,
	/// 降序。
	Desc,
}

/// 涨停股票池接口的请求参数。
///
/// 接口地址：`GET /api/a-share/special-data/limit-up-pool`。
#[derive(Debug, Serialize)]
pub struct LimitUpPoolRequest {
	/// 可选的目标交易日，Asia/Shanghai 零点的 Unix 毫秒时间戳。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date_ms: Option<i64>,
	/// 可选的页码；服务端默认值为 1。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page: Option<u32>,
	/// 可选的分页大小；服务端默认值为 50，最大为 200。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<u16>,
	/// 可选的排序字段；服务端默认按最新价排序。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort_field: Option<LimitUpSortField>,
	/// 可选的排序方向；服务端默认降序。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort_dir: Option<SortDirection>,
}

/// 涨停股票池接口的响应数据。
///
/// 接口地址：`GET /api/a-share/special-data/limit-up-pool`。
#[derive(Debug, Deserialize)]
pub struct LimitUpPoolResponse {
	/// 数据就绪时间，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 分页信息。
	pub pagination: Pagination,
	/// 涨停股票列表。
	pub item: Vec<LimitUpPoolItem>,
}

/// 通用分页信息。
#[derive(Debug, Deserialize)]
pub struct Pagination {
	/// 总条数。
	pub total: i64,
	/// 总页数。
	pub pages: i64,
	/// 当前页大小。
	pub size: i64,
	/// 当前页码。
	pub page: i64,
}

/// 单个涨停股票池条目。
#[derive(Debug, Deserialize)]
pub struct LimitUpPoolItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 六位股票代码。
	pub ticker: String,
	/// 股票简称。
	pub name: String,
	/// 是否 ST，仅供展示。
	pub is_st: bool,
	/// 是否未开板新股，仅供展示。
	pub is_new: bool,
	/// 当前价格，单位为元。
	pub last_price: f64,
	/// 涨跌幅百分比数值。
	pub price_change_ratio_pct: f64,
	/// 涨停时间，格式为 `HH:MM`。
	pub limit_up_time: String,
	/// 涨停原因；上游空字符串被服务端标准化为 `null`。
	pub limit_up_reason: Option<String>,
	/// 连板文本，例如 `首板` 或 `5天4板`。
	pub continue_day_text: String,
	/// 连板计数。
	pub continue_day_cnt: i64,
	/// 当前封单额，单位为元。
	pub seal_money: f64,
	/// 峰值封单额，单位为元。
	pub max_seal_money: f64,
}

/// 连板天梯接口的响应数据。
///
/// 接口地址：`GET /api/a-share/special-data/limit-up-ladder`。
#[derive(Debug, Deserialize)]
pub struct LimitUpLadderResponse {
	/// 数据就绪时间，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 窗口元信息。
	pub window: LimitUpLadderWindow,
	/// 按交易日组织的连板矩阵。
	pub item: Vec<LimitUpLadderDay>,
}

/// 连板天梯的窗口元信息。
#[derive(Debug, Deserialize)]
pub struct LimitUpLadderWindow {
	/// 交易日窗口长度。
	pub length: i64,
	/// 窗口内交易日列表，按服务端顺序排列。
	pub date_list: Vec<String>,
	/// 每个板位的最大列表长度。
	pub board_caps: LimitUpBoardCaps,
}

/// 连板天梯各板位的最大展示数量。
#[derive(Debug, Deserialize)]
pub struct LimitUpBoardCaps {
	/// 二连板最大条数。
	pub two_board: i64,
	/// 三连板最大条数。
	pub three_board: i64,
	/// 四连板最大条数。
	pub four_board: i64,
	/// 五连板最大条数。
	pub five_board: i64,
	/// 六连板最大条数。
	pub six_board: i64,
	/// 七板及以上最大条数。
	pub seven_over: i64,
}

/// 连板天梯中的单个交易日。
#[derive(Debug, Deserialize)]
pub struct LimitUpLadderDay {
	/// 交易日，格式为 `yyyyMMdd`。
	pub date: String,
	/// 当日各板位的股票列表。
	pub boards: LimitUpLadderBoards,
}

/// 连板天梯的六个固定板位。
#[derive(Debug, Deserialize)]
pub struct LimitUpLadderBoards {
	/// 二连板股票列表。
	#[serde(default)]
	pub two_board: Vec<LimitUpLadderStock>,
	/// 三连板股票列表。
	#[serde(default)]
	pub three_board: Vec<LimitUpLadderStock>,
	/// 四连板股票列表。
	#[serde(default)]
	pub four_board: Vec<LimitUpLadderStock>,
	/// 五连板股票列表。
	#[serde(default)]
	pub five_board: Vec<LimitUpLadderStock>,
	/// 六连板股票列表。
	#[serde(default)]
	pub six_board: Vec<LimitUpLadderStock>,
	/// 七板及以上股票列表。
	#[serde(default)]
	pub seven_over: Vec<LimitUpLadderStock>,
}

/// 连板天梯中的单只股票。
#[derive(Debug, Deserialize)]
pub struct LimitUpLadderStock {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 六位股票代码。
	pub ticker: String,
	/// 股票简称。
	pub name: String,
	/// 连板数。
	pub board_num: i64,
	/// 次一交易日是否继续封板；最近交易日没有次日参考时为 `null`。
	pub seal_nextday: Option<bool>,
	/// 上游标记等级。
	pub sign_level: i64,
}

/// 热榜的统计周期。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RankingPeriod {
	/// 日榜或 24 小时级别榜单。
	Day,
	/// 小时级别榜单。
	Hour,
}

/// 飙升榜和 A 股热股榜单共用的请求参数。
///
/// 接口地址：`GET /api/a-share/special-data/skyrocket-list` 或
/// `GET /api/a-share/special-data/hot-stock-list`。
#[derive(Debug, Serialize)]
pub struct HotRankingRequest {
	/// 可选的榜单周期；省略时服务端默认值为 `day`。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub period: Option<RankingPeriod>,
}

/// 飙升榜或 A 股热股榜单的响应数据。
#[derive(Debug, Deserialize)]
pub struct HotRankingResponse {
	/// 榜单时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 最多 30 条的榜单股票记录。
	pub item: Vec<HotRankingItem>,
}

/// 飙升榜或 A 股热股榜单中的单只股票。
#[derive(Debug, Deserialize)]
pub struct HotRankingItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 六位股票代码。
	pub ticker: String,
	/// 股票简称。
	pub name: String,
	/// 当前排名。
	pub rank: i64,
	/// 热度值，保留数据源原始字符串。
	pub heat: String,
	/// 排名变化；上游缺失时为 `null`。
	pub rank_change: Option<i64>,
	/// 排名趋势，例如 `up`、`down`、`flat` 或 `unknown`。
	pub rank_trend: String,
}

/// 历史热股排行接口的请求参数。
///
/// 接口地址：`GET /api/a-share/special-data/hot-stock-list-history`。
#[derive(Debug, Serialize)]
pub struct HotStockListHistoryRequest {
	/// 必填的目标自然日，格式为 `yyyy-MM-dd`。
	pub date: String,
}

/// 历史热股排行接口的响应数据。
///
/// 接口地址：`GET /api/a-share/special-data/hot-stock-list-history`。
#[derive(Debug, Deserialize)]
pub struct HotStockListHistoryResponse {
	/// 查询自然日，格式为 `yyyy-MM-dd`。
	pub date: String,
	/// 查询自然日 Asia/Shanghai 零点的 Unix 毫秒时间戳。
	pub date_ms: i64,
	/// 历史热股榜股票列表。
	pub item: Vec<HotStockHistoryItem>,
}

/// 历史热股榜中的单只股票。
#[derive(Debug, Deserialize)]
pub struct HotStockHistoryItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 六位股票代码。
	pub ticker: String,
	/// 股票简称。
	pub name: String,
	/// 当日热榜排名。
	pub rank: i64,
}

/// 个股热榜排名走势接口的请求参数。
///
/// 接口地址：`GET /api/a-share/special-data/hot-stock-rank-trend`。
#[derive(Debug, Serialize)]
pub struct HotStockRankTrendRequest {
	/// 单只带交易所后缀的同花顺代码。
	pub thscode: String,
	/// 必填的起始自然日，格式为 `yyyy-MM-dd`。
	pub start_date: String,
	/// 必填的结束自然日，格式为 `yyyy-MM-dd`。
	pub end_date: String,
}

/// 个股热榜排名走势接口的响应数据。
///
/// 接口地址：`GET /api/a-share/special-data/hot-stock-rank-trend`。
#[derive(Debug, Deserialize)]
pub struct HotStockRankTrendResponse {
	/// 起始自然日 Asia/Shanghai 零点的 Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 日线排名走势点位。
	pub item: Vec<HotStockRankTrendItem>,
}

/// 个股热榜排名走势中的单个日线点位。
#[derive(Debug, Deserialize)]
pub struct HotStockRankTrendItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 六位股票代码。
	pub ticker: String,
	/// 自然日，格式为 `yyyy-MM-dd`。
	pub date: String,
	/// 对应自然日 Asia/Shanghai 零点的 Unix 毫秒时间戳。
	pub date_ms: i64,
	/// 当日热榜排名。
	pub rank: i64,
}

/// 个股异动原因标签。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnomalyTag {
	/// 涨停。
	LimitUp,
	/// 跌停。
	LimitDown,
	/// 大涨。
	SharpRise,
	/// 大跌。
	SharpFall,
	/// 快速拉升。
	RapidRally,
	/// 快速下挫。
	RapidDecline,
}

/// 个股异动原因列表接口的请求参数。
///
/// 接口地址：`GET /api/a-share/special-data/anomaly-analysis-list`。
#[derive(Debug, Serialize)]
pub struct AnomalyAnalysisListRequest {
	/// 可选的逗号分隔异动标签列表；多个标签之间为 OR 关系。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tag_codes: Option<String>,
}

/// 按股票查询个股异动原因接口的请求参数。
///
/// 接口地址：`GET /api/a-share/special-data/anomaly-analysis-stock`。
#[derive(Debug, Serialize)]
pub struct AnomalyAnalysisStockRequest {
	/// 必填的逗号分隔同花顺代码列表，去重前最多 50 个代码。
	pub thscodes: String,
}

/// 个股异动原因接口的响应数据。
///
/// 接口地址：`GET /api/a-share/special-data/anomaly-analysis-list` 或
/// `GET /api/a-share/special-data/anomaly-analysis-stock`。
#[derive(Debug, Deserialize)]
pub struct AnomalyAnalysisResponse {
	/// 数据时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 个股异动原因列表。
	pub item: Vec<AnomalyAnalysisItem>,
}

/// 单条个股异动原因记录。
#[derive(Debug, Deserialize)]
pub struct AnomalyAnalysisItem {
	/// 股票名称。
	pub stock_name: String,
	/// 异动解读内容。
	pub analysis_content: String,
	/// 关键词列表；无关键词时为空数组。
	pub keyword_list: Vec<String>,
	/// 带交易所后缀的同花顺代码。
	pub thscode: String,
	/// 异动标签展示名称。
	pub tag_name: String,
}

/// 龙虎榜榜单类型。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DragonTigerBoardType {
	/// 全部榜单。
	All,
	/// 机构榜。
	Org,
	/// 游资榜。
	#[serde(rename = "hot_money")]
	HotMoney,
}

/// 龙虎榜榜单接口的请求参数。
///
/// 接口地址：`GET /api/a-share/special-data/dragon-tiger-list`。
#[derive(Debug, Serialize)]
pub struct DragonTigerListRequest {
	/// 可选的榜单类型；省略时服务端默认返回全部榜单。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub board_type: Option<DragonTigerBoardType>,
	/// 可选的目标交易日，格式为 `yyyy-MM-dd`；省略时服务端选择最近可用交易日。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date: Option<String>,
}

/// 龙虎榜榜单接口的响应数据。
///
/// 接口地址：`GET /api/a-share/special-data/dragon-tiger-list`。
#[derive(Debug, Deserialize)]
pub struct DragonTigerListResponse {
	/// 目标交易日 Asia/Shanghai 零点的 Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 实际返回的榜单类型：`all`、`org` 或 `hot_money`。
	pub board_type: String,
	/// 实际查询交易日，格式为 `yyyy-MM-dd`。
	pub trade_date: String,
	/// 上游记录数；同一股票可同时出现当日榜和三日榜。
	pub count: i64,
	/// 按股票去重后的数量。
	pub stock_count: i64,
	/// 股票维度榜单；游资榜时为空数组。
	pub stock_items: Vec<DragonTigerStockItem>,
	/// 游资维度聚合榜单；普通榜单时为空数组。
	pub hot_money_items: Vec<DragonTigerHotMoneyItem>,
}

/// 龙虎榜股票记录中的概念标签。
#[derive(Debug, Deserialize)]
pub struct DragonTigerConcept {
	/// 概念名称。
	pub name: String,
}

/// 龙虎榜股票维度记录。
#[derive(Debug, Deserialize)]
pub struct DragonTigerStockItem {
	/// 带交易所后缀的完整同花顺代码。
	pub thscode: String,
	/// 六位股票代码。
	pub ticker: String,
	/// 股票简称。
	pub name: String,
	/// 所属概念列表。
	#[serde(default)]
	pub concept_list: Vec<DragonTigerConcept>,
	/// 当日涨跌幅，小数形式。
	pub change: f64,
	/// 龙虎榜净买入金额，单位为元。
	pub net_value: f64,
	/// 龙虎榜净买入占比，小数形式。
	pub net_rate: f64,
	/// 同花顺人气排名，数值越小越靠前。
	pub hot_rank: i64,
	/// 买方金额，单位为元。
	pub buy_value: f64,
	/// 卖方金额，单位为元。
	pub sell_value: f64,
	/// 涨跌停原因。
	pub limit_reason: String,
	/// 上榜区间天数，1 表示当日榜，3 表示三日榜。
	pub range_days: i64,
	/// 机构净买入金额，单位为元。
	pub org_net_value: Option<f64>,
	/// 机构净买入占比，小数形式。
	pub org_net_rate: Option<f64>,
	/// 买入机构数。
	pub org_buy_num: Option<i64>,
	/// 卖出机构数。
	pub org_sell_num: Option<i64>,
	/// 成交金额，单位为元。
	pub amount: Option<f64>,
	/// 股票维度游资合计净买入金额，单位为元。
	pub hot_money_net_value: Option<f64>,
	/// 股票维度游资合计净买入占比，小数形式。
	pub hot_money_net_rate: Option<f64>,
	/// 当前游资在当前股票上的净买入金额，单位为元。
	pub hot_money_item_net_value: Option<f64>,
	/// 当前游资在当前股票上的净买入占比，小数形式。
	pub hot_money_item_net_rate: Option<f64>,
}

/// 龙虎榜游资维度聚合记录。
#[derive(Debug, Deserialize)]
pub struct DragonTigerHotMoneyItem {
	/// 游资名称。
	pub name: String,
	/// 聚合净买入金额，单位为元。
	pub buying: f64,
	/// 该游资关联的股票记录。
	pub rows: Vec<DragonTigerStockItem>,
}

/// 基金类型。
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FundType {
	/// 场外公募基金。
	Otc,
	/// ETF 或 LOF 等场内基金。
	Exchange,
	/// 公募 REITs。
	Reits,
}

/// 基金持有人数据的份额合并口径。
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FundMergeScope {
	/// 分别返回合并和独立份额的最新记录。
	All,
	/// 合并份额披露口径。
	Merged,
	/// 独立份额披露口径。
	Separate,
}

/// 基金持有人结构接口的请求参数。
#[derive(Debug, Serialize)]
pub struct FundHoldersDetailRequest {
	/// 基金类型。
	pub fund_type: FundType,
	/// 完整基金同花顺代码。
	pub thscode: String,
	/// 可选的份额合并口径；省略时服务端返回全部口径。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub merge_scope: Option<FundMergeScope>,
}

/// 基金持有人结构接口的响应数据。
#[derive(Debug, Deserialize)]
pub struct FundHoldersDetailResponse {
	/// 返回记录中最新的报告日，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 持有人结构记录。
	pub item: Vec<FundHoldersItem>,
}

/// 单条基金持有人结构记录。
#[derive(Debug, Deserialize)]
pub struct FundHoldersItem {
	/// 实际披露口径。
	pub merge_scope: FundMergeScope,
	/// 报告日，Unix 毫秒时间戳。
	pub report_date_ms: i64,
	/// 机构投资者占比，百分数原值。
	pub ins_position: f64,
	/// 基金份额持有人户数。
	pub holder_amount: i64,
	/// 平均每户持有基金份额。
	pub avg_holder_share: f64,
	/// 个人投资者占比，百分数原值。
	pub psnl_rate: f64,
	/// 管理人员工持有比例，百分数原值。
	pub mgmt_staff_hold_rate: f64,
}

/// 基金重仓股接口的请求参数。
#[derive(Debug, Serialize)]
pub struct FundPortfolioHoldingsRequest {
	/// 基金类型。
	pub fund_type: FundType,
	/// 完整基金同花顺代码。
	pub thscode: String,
}

/// 基金重仓股接口的响应数据。
#[derive(Debug, Deserialize)]
pub struct FundPortfolioHoldingsResponse {
	/// 数据时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 重仓股票列表。
	pub item: Vec<FundHoldingItem>,
}

/// 单条基金重仓股票记录。
#[derive(Debug, Deserialize)]
pub struct FundHoldingItem {
	/// 持仓股票的完整同花顺代码。
	pub thscode: String,
	/// 持仓股票的纯代码。
	pub ticker: String,
	/// 持仓股票名称。
	pub stock_name: String,
	/// 占基金净值比例，百分数原值。
	pub hold_ratio: f64,
}

/// 单只 ETF 行情快照接口的请求参数。
#[derive(Debug, Serialize)]
pub struct FundMarketSnapshotRequest {
	/// 单只 ETF 的完整同花顺代码。
	pub thscode: String,
}

/// 单只 ETF 行情快照接口的响应数据。
#[derive(Debug, Deserialize)]
pub struct FundMarketSnapshotResponse {
	/// 快照的上游有效时间；无有效数据时为 `null`。
	pub timestamp: Option<i64>,
	/// 行情快照记录。
	pub item: Vec<FundMarketSnapshotItem>,
}

/// 单条 ETF 行情快照记录。
#[derive(Debug, Deserialize)]
pub struct FundMarketSnapshotItem {
	/// ETF 的完整同花顺代码。
	pub thscode: String,
	/// ETF 的纯基金代码。
	pub ticker: String,
	/// 最新价。
	pub last_price: f64,
	/// 开盘价。
	pub open_price: f64,
	/// 最高价。
	pub high_price: f64,
	/// 最低价。
	pub low_price: f64,
	/// 昨收价。
	pub prev_price: f64,
	/// 涨跌幅，百分数原值。
	pub price_change_ratio_pct: f64,
	/// 涨跌额。
	pub price_change: f64,
	/// 振幅，百分数原值。
	pub price_amplitude_ratio_pct: f64,
	/// 成交量。
	pub volume: f64,
	/// 成交额。
	pub turnover: f64,
	/// 换手率，百分数原值。
	pub turnover_ratio_pct: f64,
}

/// 单只 ETF 历史日 K 线接口的请求参数。
#[derive(Debug, Serialize)]
pub struct FundMarketHistoricalRequest {
	/// 单只 ETF 的完整同花顺代码。
	pub thscode: String,
	/// K 线周期；省略时服务端默认日 K。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interval: Option<PriceInterval>,
	/// 查询起始时间，Unix 毫秒时间戳。
	pub start: i64,
	/// 查询结束时间，Unix 毫秒时间戳。
	pub end: i64,
}

/// 单只 ETF 历史日 K 线接口的响应数据。
#[derive(Debug, Deserialize)]
pub struct FundMarketHistoricalResponse {
	/// 数据就绪时间，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 请求中的 ETF 同花顺代码。
	pub thscode: String,
	/// K 线周期，当前固定为 `1d`。
	pub interval: PriceInterval,
	/// ETF 当前没有复权语义，服务端固定返回 `null`。
	pub adjust: Option<PriceAdjust>,
	/// 历史日 K 线记录。
	pub item: Vec<PriceBarItem>,
}

/// 基金净值查询的时间范围。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FundNavRange {
	/// 近一周。
	Week,
	/// 近一月。
	Month,
	/// 近三月。
	Tmonth,
	/// 近半年。
	Hyear,
	/// 近一年。
	Year,
	/// 近两年。
	Twoyear,
	/// 近三年。
	Tyear,
	/// 近五年。
	Fyear,
}

/// 基金净值类型。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum FundNavType {
	/// 单位净值。
	#[serde(rename = "unit")]
	Unit,
	/// 复权净值。
	#[serde(rename = "adj")]
	Adjusted,
	/// 同时返回单位净值和复权净值。
	#[serde(rename = "unit,adj")]
	Both,
}

/// 基金净值接口的请求参数。
#[derive(Debug, Serialize)]
pub struct FundPerformanceNavRequest {
	/// 基金类型。
	pub fund_type: FundType,
	/// 完整基金同花顺代码。
	pub thscode: String,
	/// 可选的净值时间范围；省略时仅返回最新一条。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub range: Option<FundNavRange>,
	/// 可选的净值类型；省略时由服务端返回两种净值。
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nav_type: Option<FundNavType>,
}

/// 基金净值接口的响应数据。
#[derive(Debug, Deserialize)]
pub struct FundPerformanceNavResponse {
	/// 数据时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 净值记录。
	pub item: Vec<FundNavItem>,
}

/// 单条基金净值记录。
#[derive(Debug, Deserialize)]
pub struct FundNavItem {
	/// 净值日期，Unix 毫秒时间戳。
	pub nav_date: i64,
	/// 单位净值；未请求时不返回。
	pub unit_nav: Option<f64>,
	/// 复权净值；未请求时不返回。
	pub adj_nav: Option<f64>,
}

/// 基金区间收益接口的请求参数。
#[derive(Debug, Serialize)]
pub struct FundPerformanceReturnsRequest {
	/// 基金类型。
	pub fund_type: FundType,
	/// 完整基金同花顺代码。
	pub thscode: String,
}

/// 基金区间收益接口的响应数据。
#[derive(Debug, Deserialize)]
pub struct FundPerformanceReturnsResponse {
	/// 数据时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 区间收益记录。
	pub item: Vec<FundReturnsItem>,
}

/// 单条基金区间收益记录。
#[derive(Debug, Deserialize)]
pub struct FundReturnsItem {
	/// 近一月收益率，百分数原值。
	pub return_month: Option<f64>,
	/// 近三月收益率，百分数原值。
	pub return_tmonth: Option<f64>,
	/// 近半年收益率，百分数原值。
	pub return_hyear: Option<f64>,
	/// 近一年收益率，百分数原值。
	pub return_year: Option<f64>,
	/// 近三年收益率，百分数原值。
	pub return_tyear: Option<f64>,
	/// 近五年收益率，百分数原值。
	pub return_fyear: Option<f64>,
	/// 今年以来收益率，百分数原值。
	pub return_nowyear: Option<f64>,
	/// 成立以来收益率，百分数原值。
	pub return_now: Option<f64>,
}

/// 基金基本资料接口的请求参数。
#[derive(Debug, Serialize)]
pub struct FundProfileDetailRequest {
	/// 基金类型。
	pub fund_type: FundType,
	/// 完整基金同花顺代码。
	pub thscode: String,
}

/// 基金基本资料接口的响应数据。
#[derive(Debug, Deserialize)]
pub struct FundProfileDetailResponse {
	/// 数据时间戳，Unix 毫秒时间戳。
	pub timestamp: i64,
	/// 基金基本资料记录。
	pub item: Vec<FundProfileItem>,
}

/// 单条基金基本资料记录。
#[derive(Debug, Deserialize)]
pub struct FundProfileItem {
	/// 完整基金同花顺代码。
	pub thscode: String,
	/// 纯基金代码。
	pub ticker: String,
	/// 基金名称。
	pub fund_name: Option<String>,
	/// 成立日期，Unix 毫秒时间戳。
	pub estab_date: Option<i64>,
	/// 基金管理人名称。
	pub mgmt_name: Option<String>,
	/// 基金经理姓名。
	pub manager_name: Option<String>,
}

#[cfg(test)]
mod tests {
	use super::{
		ASharePricesHistoricalRequest, ASharePricesHistoricalResponse, ASharePricesSnapshotRequest,
		DragonTigerBoardType, DragonTigerListRequest, DragonTigerListResponse,
		FinancialIndicatorsResponse, FinancialPeriod, FinancialStatementsRequest,
		FinancialStatementsResponse, IncomeStatementItem, IndexPricesHistoricalResponse,
		LimitUpLadderResponse, LimitUpPoolRequest, LimitUpSortField, MarketDumpDownloadUrlResponse,
		PriceAdjust, PriceInterval, PricesSnapshotResponse, SortDirection, ThsIndexListRequest,
		ThsIndexTag, TickerAssetType, TickerListRequest, TickerSearchRequest, TickerSearchResponse,
		TradingDaysResponse,
	};

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
	fn serializes_fund_requests() {
		let nav = super::FundPerformanceNavRequest {
			fund_type: super::FundType::Exchange,
			thscode: "510300.SH".to_string(),
			range: None,
			nav_type: Some(super::FundNavType::Both),
		};
		assert_eq!(
			serde_urlencoded::to_string(nav).unwrap(),
			"fund_type=exchange&thscode=510300.SH&nav_type=unit%2Cadj"
		);

		let historical = super::FundMarketHistoricalRequest {
			thscode: "510300.SH".to_string(),
			interval: Some(PriceInterval::Day1),
			start: 1_626_451_200_000,
			end: 1_784_217_600_000,
		};
		assert_eq!(
			serde_urlencoded::to_string(historical).unwrap(),
			"thscode=510300.SH&interval=1d&start=1626451200000&end=1784217600000"
		);
	}

	#[test]
	fn deserializes_fund_responses() {
		let holders: super::FundHoldersDetailResponse = serde_json::from_str(
			r#"{
				"timestamp": 1767110400000,
				"item": [{
					"merge_scope": "merged",
					"report_date_ms": 1609344000000,
					"ins_position": 0.18,
					"holder_amount": 7058156,
					"avg_holder_share": 4819.42,
					"psnl_rate": 99.82,
					"mgmt_staff_hold_rate": 0.0062
				}]
			}"#,
		)
		.unwrap();
		assert_eq!(holders.item[0].holder_amount, 7_058_156);

		let profile: super::FundProfileDetailResponse = serde_json::from_str(
			r#"{
				"timestamp": 1784210313786,
				"item": [{
					"thscode": "025480.OF",
					"ticker": "025480",
					"fund_name": "沪深300A",
					"estab_date": null,
					"mgmt_name": null,
					"manager_name": null
				}]
			}"#,
		)
		.unwrap();
		assert_eq!(profile.item[0].fund_name.as_deref(), Some("沪深300A"));
		assert_eq!(profile.item[0].estab_date, None);

		let holdings: super::FundPortfolioHoldingsResponse = serde_json::from_str(
			r#"{"timestamp":0,"item":[{
				"thscode":"300750.SZ","ticker":"300750","stock_name":"宁德时代","hold_ratio":4.67
			}]}"#,
		)
		.unwrap();
		assert_eq!(holdings.item[0].stock_name, "宁德时代");

		let snapshot: super::FundMarketSnapshotResponse = serde_json::from_str(
			r#"{"timestamp":1784210584000,"item":[{
				"thscode":"510300.SH","ticker":"510300","last_price":4.753,"open_price":4.775,
				"high_price":4.825,"low_price":4.724,"prev_price":4.838,
				"price_change_ratio_pct":-1.756924,"price_change":-0.085,
				"price_amplitude_ratio_pct":2.08764,"volume":1657822800,"turnover":7909234100,
				"turnover_ratio_pct":9.012068
			}]}"#,
		)
		.unwrap();
		assert_eq!(snapshot.item[0].turnover_ratio_pct, 9.012068);

		let historical: super::FundMarketHistoricalResponse = serde_json::from_str(
			r#"{"timestamp":1784131200000,"thscode":"510300.SH","interval":"1d","adjust":null,"item":[{
				"date_ms":1626624000000,"open_price":4.728,"high_price":4.769,"low_price":4.687,
				"close_price":4.759,"volume":332046160,"turnover":1709347700
			}]}"#,
		)
		.unwrap();
		assert_eq!(historical.adjust, None);

		let nav: super::FundPerformanceNavResponse = serde_json::from_str(
			r#"{"timestamp":1784131200000,"item":[{"nav_date":1752595200000,"unit_nav":4.0713}]}"#,
		)
		.unwrap();
		assert_eq!(nav.item[0].adj_nav, None);

		let returns: super::FundPerformanceReturnsResponse = serde_json::from_str(
			r#"{"timestamp":0,"item":[{"return_month":-3.33,"return_year":19.66}]}"#,
		)
		.unwrap();
		assert_eq!(returns.item[0].return_tmonth, None);
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

	#[test]
	fn serializes_documented_enum_values() {
		let historical = ASharePricesHistoricalRequest {
			thscode: "600519.SH".to_string(),
			interval: PriceInterval::Day1,
			start: 1_716_105_600_000,
			end: 1_747_648_000_000,
			adjust: Some(PriceAdjust::Forward),
			offset: Some(20),
		};

		assert_eq!(
			serde_urlencoded::to_string(historical).unwrap(),
			"thscode=600519.SH&interval=1d&start=1716105600000&end=1747648000000&adjust=forward&offset=20"
		);

		let index_list = ThsIndexListRequest { tag: Some(ThsIndexTag::CnConcept) };
		assert_eq!(serde_urlencoded::to_string(index_list).unwrap(), "tag=cn_concept");
	}

	#[test]
	fn deserializes_nullable_financial_indicator_values() {
		let response: FinancialIndicatorsResponse = serde_json::from_str(
			r#"{
				"thscode": "300033.SZ",
				"report": "2025-1",
				"abilities": [{
					"ability": "profitability",
					"indicators": [{
						"index_id": "earned_interest_multiple",
						"value": null
					}]
				}]
			}"#,
		)
		.unwrap();

		assert_eq!(response.abilities[0].ability, "profitability");
		assert_eq!(response.abilities[0].indicators[0].value, None);
	}

	#[test]
	fn serializes_pagination_financial_and_special_data_parameters() {
		let ticker_list = TickerListRequest {
			asset_type: Some(TickerAssetType::AShare),
			limit: Some(1_000),
			offset: Some(2_000),
		};
		assert_eq!(
			serde_urlencoded::to_string(ticker_list).unwrap(),
			"asset_type=a-share&limit=1000&offset=2000"
		);

		let snapshot = ASharePricesSnapshotRequest {
			thscodes: Some("600519.SH,000001.SZ".to_string()),
			limit: Some(100),
			offset: Some(50),
		};
		assert_eq!(
			serde_urlencoded::to_string(snapshot).unwrap(),
			"thscodes=600519.SH%2C000001.SZ&limit=100&offset=50"
		);

		let financials = FinancialStatementsRequest {
			thscode: "600519.SH".to_string(),
			period: FinancialPeriod::Annual,
			limit: None,
			start: Some(1_577_808_000_000),
			end: Some(1_735_574_400_000),
		};
		assert_eq!(
			serde_urlencoded::to_string(financials).unwrap(),
			"thscode=600519.SH&period=annual&start=1577808000000&end=1735574400000"
		);

		let limit_up = LimitUpPoolRequest {
			date_ms: Some(1_748_102_400_000),
			page: Some(2),
			size: Some(100),
			sort_field: Some(LimitUpSortField::LimitUpTime),
			sort_dir: Some(SortDirection::Desc),
		};
		assert_eq!(
			serde_urlencoded::to_string(limit_up).unwrap(),
			"date_ms=1748102400000&page=2&size=100&sort_field=limit_up_time&sort_dir=desc"
		);

		let dragon_tiger = DragonTigerListRequest {
			board_type: Some(DragonTigerBoardType::HotMoney),
			date: Some("2026-07-01".to_string()),
		};
		assert_eq!(
			serde_urlencoded::to_string(dragon_tiger).unwrap(),
			"board_type=hot_money&date=2026-07-01"
		);
	}

	#[test]
	fn deserializes_price_and_market_dump_responses() {
		let snapshot: PricesSnapshotResponse = serde_json::from_str(
			r#"{
				"timestamp": null,
				"total": 1,
				"item": [{
					"thscode": "600519.SH",
					"ticker": "600519",
					"last_price": 1277.8,
					"price_change": 21.8,
					"price_change_ratio_pct": 1.735669,
					"open_price": 1252.08,
					"high_price": 1282,
					"low_price": 1250.21,
					"prev_price": 1256,
					"volume": 3098875,
					"turnover": 3937375200
				}]
			}"#,
		)
		.unwrap();
		assert_eq!(snapshot.timestamp, None);
		assert_eq!(snapshot.item[0].volume, 3_098_875.0);

		let historical: ASharePricesHistoricalResponse = serde_json::from_str(
			r#"{
				"timestamp": 1747584000000,
				"item": [{
					"date_ms": 1716134400000,
					"open_price": 1611.602,
					"high_price": 1626.602,
					"low_price": 1601.722,
					"close_price": 1602.612,
					"volume": 3142572,
					"turnover": 5401389334.87
				}]
			}"#,
		)
		.unwrap();
		assert_eq!(historical.item[0].date_ms, 1_716_134_400_000);

		let download: MarketDumpDownloadUrlResponse = serde_json::from_str(
			r#"{
				"presigned_url": "https://download.example/file.parquet",
				"presigned_url_expires_at": "2026-07-17T10:29:57.827304315+08:00",
				"expires_in_seconds": 300
			}"#,
		)
		.unwrap();
		assert_eq!(download.presigned_url, "https://download.example/file.parquet");
		assert_eq!(download.presigned_url_expires_at, "2026-07-17T10:29:57.827304315+08:00");
		assert_eq!(download.expires_in_seconds, 300);
	}

	#[test]
	fn deserializes_flattened_financial_statement_metadata_and_null_amounts() {
		let response: FinancialStatementsResponse<IncomeStatementItem> = serde_json::from_str(
			r#"{
				"timestamp": 1735574400000,
				"item": [{
					"thscode": "600519.SH",
					"ticker": "600519",
					"period": "annual",
					"fiscal_year": 2024,
					"fiscal_period": "FY",
					"report_date_ms": 1735574400000,
					"period_end_ms": 1735574400000,
					"currency": "CNY",
					"operating_income": 174144000000,
					"operating_costs": null,
					"operating_expenses": 50000000000,
					"sales_fee": 5000000000,
					"manage_fee": 9000000000,
					"research_and_development_expenses": 150000000,
					"operating_profit": 124000000000,
					"interest_expenses": 0,
					"profit_total": 124000000000,
					"income_tax_expense": 31000000000,
					"net_profit": 93000000000,
					"parent_holder_net_profit": 86000000000,
					"basic_eps": 68.5
				}]
			}"#,
		)
		.unwrap();

		let item = &response.item[0];
		assert_eq!(item.metadata.fiscal_period, "FY");
		assert_eq!(item.metadata.period, FinancialPeriod::Annual);
		assert_eq!(item.operating_costs, None);
		assert_eq!(item.basic_eps, Some(68.5));
	}

	#[test]
	fn deserializes_index_calendar_and_special_data_responses() {
		let index: IndexPricesHistoricalResponse = serde_json::from_str(
			r#"{
				"timestamp": 1747584000000,
				"adjust": null,
				"item": [{
					"date_ms": 1716134400000,
					"open_price": 3108.22,
					"high_price": 3125.74,
					"low_price": 3101.15,
					"close_price": 3120.68,
					"volume": 281000000,
					"turnover": 360000000000
				}]
			}"#,
		)
		.unwrap();
		assert_eq!(index.adjust, None);
		assert_eq!(index.item[0].close_price, 3120.68);

		let calendar: TradingDaysResponse = serde_json::from_str(
			r#"{"timestamp":1748102400000,"item":[{"date_ms":1716566400000,"date":"20250525"}]}"#,
		)
		.unwrap();
		assert_eq!(calendar.item[0].date, "20250525");

		let ladder: LimitUpLadderResponse = serde_json::from_str(
			r#"{
				"timestamp": 1748102400000,
				"window": {
					"length": 30,
					"date_list": ["20250620"],
					"board_caps": {
						"two_board": 4,
						"three_board": 4,
						"four_board": 4,
						"five_board": 4,
						"six_board": 4,
						"seven_over": 4
					}
				},
				"item": [{
					"date": "20250620",
					"boards": {
						"two_board": [{
							"thscode": "603986.SH",
							"ticker": "603986",
							"name": "兆易创新",
							"board_num": 2,
							"seal_nextday": null,
							"sign_level": 1
						}],
						"three_board": [],
						"four_board": [],
						"five_board": [],
						"six_board": [],
						"seven_over": []
					}
				}]
			}"#,
		)
		.unwrap();
		assert_eq!(ladder.item[0].boards.two_board[0].seal_nextday, None);

		let dragon_tiger: DragonTigerListResponse = serde_json::from_str(
			r#"{
				"timestamp": 1782921600000,
				"board_type": "all",
				"trade_date": "2026-07-01",
				"count": 1,
				"stock_count": 1,
				"stock_items": [{
					"thscode": "002407.SZ",
					"ticker": "002407",
					"name": "多氟多",
					"change": 0.09994,
					"net_value": 1786253128.23,
					"net_rate": 0.11901893,
					"hot_rank": 2,
					"buy_value": 2674755016.05,
					"sell_value": 888501887.82,
					"limit_reason": "半导体",
					"range_days": 3
				}],
				"hot_money_items": []
			}"#,
		)
		.unwrap();
		assert!(dragon_tiger.stock_items[0].concept_list.is_empty());
		assert_eq!(dragon_tiger.stock_items[0].org_net_value, None);
	}
}
