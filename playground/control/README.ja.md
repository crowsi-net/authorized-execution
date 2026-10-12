# 制御境界のplayground

owner repoだけで以下を実行します。CoelaReleaseや兄弟sourceのcheckoutは不要です。

```sh
cargo run --locked -p crowsi-policy-engine --example policy_decisions
cargo test --locked -p crowsi-enforcement-point --test v2_failure_semantics --test fence_consumption --test adapter_boundary
cargo test --locked -p crowsi-independent-verifier
cargo test --locked -p crowsi-rescue-console
```

実行例は公開policy APIへ同repoの架空契約fixtureを渡し、許可判定・失効・証跡不足の拒否を確認します。署名は架空です。純粋policy評価は署名認証・認可発行・effect実行ではありません。別のenforcement testが署名command/lease束縛、replay/失敗、注入fake adapterを検証します。独立verifierは別取得read-backを求め、復旧は別trust経路を使います。実authority・cloud操作・Windows CNG受入は行いません。

## 責務と配布

現workspaceは契約・policy・署名認可・transport・実行・独立検証・復旧のcrateを分けています。契約を直接利用するcrateが複数あるため名前だけで削除しません。署名wire形式の整理はvalidator・正規化bytes・producer/consumer・永続ledgerの検証を伴います。未知field・署名・期限・対象束縛・replayの検査は維持します。

最終物理配置の判断までは現ownerを維持し、`publish = false`のcrateを公開済みとしません。libraryとnative helperは別配布です。Nodeのメモリ内制御で署名付き永続native認可を代替しません。

## 有効化・復旧の記録

環境ownerがauthority/operator identity、account/resource/action、署名入力、独立read-backを指定します。source/archive/native digest・platform/ABI・backend・trust anchor・ledger・backup/restoreを記録します。backend/入力の不足をfallbackで隠しません。不明/部分成功の証跡を保持して対象grantを失効し、実行された可能性のある操作を無制限の代替grantでretryしません。復旧identityと経路は通常authorityから分け、Windows helper利用前に隔離Windows/WSL受入を完了します。

[English](README.md)
