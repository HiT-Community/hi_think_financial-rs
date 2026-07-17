# 同花顺金融数据 Rust SDK

本项目是社区维护的 Rust 客户端库，用于调用[同花顺金融数据 API](https://fuyao.aicubes.cn/docs/)。它不是官方 SDK；接口能力、数据范围、访问权限和 API Key 均以官方服务文档为准。

客户端默认连接 `https://fuyao.aicubes.cn`，自动携带 `X-api-key` 请求头，并将服务端统一的 `ApiResponse` 响应转换为 Rust 类型和错误。

## 提供的接口

- 标的检索与代码表
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
