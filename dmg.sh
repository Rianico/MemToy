#!/bin/bash

app_name=MemToy
# 创建 .app 目录结构
mkdir -p dist/$app_name.app/Contents/{MacOS,Resources}

# 复制可执行文件
cp target/release/$app_name dist/$app_name.app/Contents/MacOS/$app_name

# 创建 Info.plist 文件
cat <<EOF > dist/$app_name.app/Contents/Info.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$app_name</string>
    <key>CFBundleIdentifier</key>
    <string>tech.rianico.$app_name</string>
    <key>CFBundleName</key>
    <string>$app_name</string>
    <key>CFBundleVersion</key>
    <string>0.2.1</string>
    <key>CFBundleShortVersionString</key>
    <string>0.2.1</string>
</dict>
</plist>
EOF

# 创建 DMG 文件
hdiutil create -srcfolder dist/$app_name.app -volname "$app_name" -fs HFS+ -format UDZO -ov dist/$app_name.dmg

echo "DMG 文件已生成：dist/$app_name.dmg"
