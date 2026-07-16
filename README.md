# 同花顺金融数据 Rust SDK

本项目是社区维护的 Rust 客户端库，用于调用[同花顺金融数据 API](https://fuyao.aicubes.cn/docs/)。它不是官方 SDK；接口能力、数据范围、访问权限和 API Key 均以官方服务文档为准。

客户端默认连接 `https://fuyao.aicubes.cn`，自动携带 `X-api-key` 请求头，并将服务端统一的 `ApiResponse` 响应转换为 Rust 类型和错误。当前已覆盖标的检索、A 股行情与历史 K 线、除复权、财务报表、交易日历、指数、涨跌停、热榜、异动原因、龙虎榜及全市场数据导出等接口。

## 文档

- [官方文档](https://fuyao.aicubes.cn/docs/)
- [快速开始与 API Key](https://fuyao.aicubes.cn/docs/quickstart/)
- [REST API 参考](https://fuyao.aicubes.cn/docs/api-reference/overview/)

## 联系我

- [Bilibili](https://space.bilibili.com/3546892086544493)
