# iDemand

个人需求管理工具，方便快捷管理自己所承接的需求。

## 功能

- [x] 新建需求
- [x] 需求桌面显示
- [x] 需求内容：名字、描述、上线时间、需求链接地址、设计稿地址、本地项目地址、需求目前状态<待开发、开发中、已完成、已发布>
- [ ] 支持显示还有多少时间上线
- [x] 支持搜索需求，更具条件搜索
- [ ] 支持按照时间生成周报，周报提醒-后续支持
- [x] 支持快速复制需求整合信息，发送别人
- [ ] 支持报表-后续支持
- [x] 桌面小窗显示，状态，需求名称，打开文档、打开设计稿、打开本地项目
- [ ] 支持多个项目-待考虑
- [x] 多个链接
- [x] 支持vscode、sublimetext、webstrom 打开项目
- [x] 额外链接，支持添加额外链接在下边平铺 点击打开
- [ ] 支持快捷键唤起窗口搜索 - 待考虑
- [x] 支持单个需求固定到桌面显示
- [ ] 支持导入、导出



周报信息
- 本周完成了x个需求，新增了x个需求，还有x个需求待完成，加油💪🏻Fighting！！！
- 本周还有x个需求未完成，下周继续努力！！！


### 新建需求

表单项：
名字 - name - string
描述 - desc - string
发布时间 - publish_date - datetime


## 通用平台打包发布

```shell
# 打包
yarn tauri build --target universal-apple-darwin
# 签名生成 .pkg 文件
productbuild --component ./src-tauri/target/universal-apple-darwin/release/bundle/macos/iHost.app /Applications ./pkg/iHost_all.pkg --sign "3rd Party Mac Developer Installer: Bingkui Kang (N8CP79P49X)" --product ./entitlements.plist
```

```shell
# 第一步
TARGET_PATH_BASE=./src-tauri/target/universal-apple-darwin/release/bundle/macos/iDemand
APP_SIGN_IDENTITY="3rd Party Mac Developer Application: Bingkui Kang (N8CP79P49X)"
INSTALLER_SIGN_IDENTITY="3rd Party Mac Developer Installer: Bingkui Kang (N8CP79P49X)"
ENTITLEMENTS_PLIST=./entitlements.plist

plutil -convert xml1 $ENTITLEMENTS_PLIST
codesign --force --verbose --deep --sign $APP_SIGN_IDENTITY  ./$TARGET_PATH_BASE.app
codesign --force --verbose --deep --sign $APP_SIGN_IDENTITY --entitlements $ENTITLEMENTS_PLIST ./$TARGET_PATH_BASE.app
productbuild --component $TARGET_PATH_BASE.app /Applications $TARGET_PATH_BASE.pkg --sign $INSTALLER_SIGN_IDENTITY --product $ENTITLEMENTS_PLIST
```
