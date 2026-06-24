> Migrated from `docs/standard-alignment-audit.md` on 2026-06-24.
> Owner: SDKWork maintainers

# SDKWork GitHub 标准对齐审计

最后更新：2026-06-23

## 总体结论

| 维度 | 状态 | 说明 |
| --- | --- | --- |
| sdkwork-specs 字典与目录结构 | 已对齐 | 标准根目录、`AGENTS.md`、`.sdkwork/`、`specs/` |
| sdkwork-web-framework | 已对齐 | `WebFrameworkLayer` + `HttpRouteManifest` |
| sdkwork-database | 已对齐 | `database.manifest.json`、migrations、API server bootstrap |
| sdkwork-utils | 已对齐 | Rust `sdkwork-utils-rust` + PC `@sdkwork/utils` |
| sdkwork-discovery | 不适用 | 当前无 gRPC/RPC 服务 |
| 部署与打包 | 已对齐 | `sdkwork.workflow.json` + 薄 GitHub workflow |

## 验证命令

```bash
pnpm check
pnpm verify
node --test scripts/verify-github-standard-architecture.test.mjs
```

