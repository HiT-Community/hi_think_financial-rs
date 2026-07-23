# 同花顺金融数据 Rust SDK

本项目是社区维护的 Rust 客户端库，用于调用[同花顺金融数据 API](https://fuyao.aicubes.cn/docs/)。它不是官方 SDK；接口能力、数据范围、访问权限和 API Key 均以官方服务文档为准。

客户端默认连接 `https://fuyao.aicubes.cn`，自动携带 `X-api-key` 请求头，并将服务端统一的 `ApiResponse` 响应转换为 Rust 类型和错误。

## 安装

推荐使用 `cargo add` 添加依赖：

```bash
cargo add hi_think_financial
cargo add tokio --features macros,rt-multi-thread
```

添加后 `Cargo.toml` 中会包含类似配置：

```toml
[dependencies]
hi_think_financial = "0.1.6"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## API Key

先从[快速开始文档](https://fuyao.aicubes.cn/docs/quickstart/)获取 API Key，然后在本地环境变量中配置：

```powershell
$env:FUYAO_API_KEY = "你的 API Key"
```

Linux/macOS:

```bash
export FUYAO_API_KEY="你的 API Key"
```

## 快速开始

下面是一个完整的可运行示例，用关键词搜索标的并打印匹配结果：

```rust
use hi_think_financial::{HiThinkFinancialClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("FUYAO_API_KEY")?;
    let client = HiThinkFinancialClient::new(&api_key);

    let result = client
        .ticker_search(types::TickerSearchRequest {
            q: "贵州茅台".to_string(),
            asset_type: Some(types::TickerAssetType::AShare),
            limit: Some(10),
        })
        .await?;

    for item in result.item {
        println!("{} {} {}", item.thscode, item.ticker, item.name);
    }

    Ok(())
}
```

运行：

```bash
cargo run
```

## 使用方式

这个 crate 的核心用法是：

1. 用 `HiThinkFinancialClient::new(api_key)` 创建客户端。
2. 从 `hi_think_financial::types` 中选择对应的请求结构体和枚举。
3. 调用客户端上的异步方法，例如 `ticker_search`、`a_share_prices_snapshot`、`income_statements`。
4. 使用 `.await?` 取得强类型响应。

所有接口方法都会返回 `anyhow::Result<T>`。HTTP 请求失败、HTTP 状态码非 2xx、接口业务 `code != 0` 或成功响应缺少 `data` 时都会返回错误。

## 常用案例

### 查询 A 股实时快照

```rust
use hi_think_financial::{HiThinkFinancialClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HiThinkFinancialClient::new(&std::env::var("FUYAO_API_KEY")?);

    let snapshot = client
        .a_share_prices_snapshot(types::ASharePricesSnapshotRequest {
            thscodes: Some("600519.SH,000001.SZ".to_string()),
            limit: None,
            offset: None,
        })
        .await?;

    for item in snapshot.item {
        println!(
            "{} 最新价: {}, 涨跌幅: {}%",
            item.thscode, item.last_price, item.price_change_ratio_pct
        );
    }

    Ok(())
}
```

### 查询历史日 K

时间参数使用 Unix 毫秒时间戳。

```rust
use hi_think_financial::{HiThinkFinancialClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HiThinkFinancialClient::new(&std::env::var("FUYAO_API_KEY")?);

    let bars = client
        .a_share_prices_historical(types::ASharePricesHistoricalRequest {
            thscode: "600519.SH".to_string(),
            interval: types::PriceInterval::Day1,
            start: 1_716_105_600_000,
            end: 1_747_648_000_000,
            adjust: Some(types::PriceAdjust::Forward),
            offset: None,
        })
        .await?;

    for bar in bars.item.iter().take(5) {
        println!("{} close={}", bar.date_ms, bar.close_price);
    }

    Ok(())
}
```

### 查询财务报表

```rust
use hi_think_financial::{HiThinkFinancialClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HiThinkFinancialClient::new(&std::env::var("FUYAO_API_KEY")?);

    let income = client
        .income_statements(types::FinancialStatementsRequest {
            thscode: "600519.SH".to_string(),
            period: types::FinancialPeriod::Annual,
            limit: Some(4),
            start: None,
            end: None,
        })
        .await?;

    for report in income.item {
        println!(
            "{} {} revenue={:?} net_profit={:?}",
            report.metadata.fiscal_year,
            report.metadata.fiscal_period,
            report.operating_income,
            report.net_profit
        );
    }

    Ok(())
}
```

### 查询指数列表与成分股

```rust
use hi_think_financial::{HiThinkFinancialClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HiThinkFinancialClient::new(&std::env::var("FUYAO_API_KEY")?);

    let indexes = client
        .ths_index_list(types::ThsIndexListRequest {
            tag: Some(types::ThsIndexTag::Industry),
        })
        .await?;

    if let Some(index) = indexes.item.first() {
        let constituents = client
            .ths_index_constituents(types::ThsIndexConstituentsRequest {
                thscode: index.thscode.clone(),
            })
            .await?;

        println!("{} 成分股数量: {}", index.name, constituents.item.len());
    }

    Ok(())
}
```

### 获取全市场 Parquet 下载链接

```rust
use hi_think_financial::HiThinkFinancialClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HiThinkFinancialClient::new(&std::env::var("FUYAO_API_KEY")?);

    let dump = client.market_dump_daily_k_download_url().await?;
    println!("download: {}", dump.presigned_url);
    println!("expires in: {} seconds", dump.expires_in_seconds);

    Ok(())
}
```

### 查询特殊数据

```rust
use hi_think_financial::{HiThinkFinancialClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HiThinkFinancialClient::new(&std::env::var("FUYAO_API_KEY")?);

    let hot = client
        .hot_stock_list(types::HotRankingRequest {
            period: Some(types::RankingPeriod::Day),
        })
        .await?;

    for item in hot.item.iter().take(10) {
        println!("#{} {} {}", item.rank, item.thscode, item.name);
    }

    let dragon_tiger = client
        .dragon_tiger_list(types::DragonTigerListRequest {
            board_type: Some(types::DragonTigerBoardType::All),
            date: None,
        })
        .await?;

    println!(
        "{} 龙虎榜股票数: {}",
        dragon_tiger.trade_date, dragon_tiger.stock_count
    );

    Ok(())
}
```

## 参数约定

- `thscode` 使用带交易所后缀的同花顺代码，例如 `600519.SH`、`000001.SZ`。
- 多标的接口使用英文逗号拼接，例如 `600519.SH,000001.SZ`。
- 时间区间参数通常使用 Unix 毫秒时间戳，日期参数通常使用 `yyyy-MM-dd` 字符串；具体以官方接口文档为准。
- 请求结构体中的 `Option` 字段传 `None` 时会省略该查询参数，由服务端使用默认值。

## 提供的接口

- 标的检索与代码表
- 基金资料、重仓股、净值、收益、持有人结构与 ETF 行情
- A 股行情快照与历史 K 线
- 全市场日 K、近十日 K 线与复权因子数据导出
- 除复权因子
- 利润表、资产负债表、现金流量表与财务指标
- A 股交易日历
- 同花顺指数列表、指数成分股、指数行情快照与历史 K 线
- 涨停池与连板梯队
- 热度飙升榜、热股榜、历史热股榜与热榜排名走势
- 个股异动原因列表与批量查询
- 龙虎榜

## 文档

- [官方文档](https://fuyao.aicubes.cn/docs/)
- [快速开始与 API Key](https://fuyao.aicubes.cn/docs/quickstart/)
- [REST API 参考](https://fuyao.aicubes.cn/docs/api-reference/overview/)

## 联系我

- [Bilibili](https://space.bilibili.com/3546892086544493)

## 扫码加群

![同花顺金融数据社区群二维码](docs/image/hi_think_financial_community.png)
